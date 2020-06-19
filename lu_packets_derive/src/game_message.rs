use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Field, Fields, Meta, NestedMeta, Type};

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let des_impl_generics = &mut input.generics.clone();
	des_impl_generics.params.push(parse_quote!(__READER: ::std::io::Read));
	let (des_impl_generics, _, _) = des_impl_generics.split_for_impl();

	let ser_impl_generics = &mut input.generics.clone();
	ser_impl_generics.params.push(parse_quote!('__LIFETIME));
	ser_impl_generics.params.push(parse_quote!(__WRITER: ::std::io::Write));
	let (ser_impl_generics, _, _) = ser_impl_generics.split_for_impl();

	let (_, ty_generics, where_clause) = input.generics.split_for_impl();

	let data = match &input.data {
		Data::Struct(data) => data,
		_ => unimplemented!(),
	};

	let deser_code = gen_deser_code(&data.fields);
	let ser_code = gen_ser_code(&data.fields);
	(quote! {
		impl #des_impl_generics ::endio::Deserialize<::endio::LE, __READER> for #name #ty_generics #where_clause {
			fn deserialize(reader: &mut __READER) -> ::std::io::Result<Self> {
				#deser_code
			}
		}

		impl #ser_impl_generics ::endio::Serialize<::endio::LE, __WRITER> for &'__LIFETIME #name #ty_generics #where_clause {
			fn serialize(self, writer: &mut __WRITER) -> ::std::io::Result<()> {
				#ser_code
			}
		}
	}).into()
}

fn gen_deser_code(fields: &Fields) -> TokenStream {
	let fields = match fields {
		Fields::Named(fields) => fields,
		_ => unimplemented!(),
	};
	let mut msg_needs_bitreader = false;
	let mut idents = vec![];
	let mut deser = vec![];
	for f in &fields.named {
		let ident = &f.ident;
		idents.push(quote! { #ident, });

		let is_bool = match &f.ty {
			Type::Path(path) => path.path.is_ident("bool"),
			_ => false,
		};
		let default = get_gm_default(&f);
		let field_needs_bitreader = is_bool || default.is_some();
		let create_bitreader = if !msg_needs_bitreader && field_needs_bitreader {
			msg_needs_bitreader = true;
			quote! { let mut reader = &mut ::endio_bit::BEBitReader::new(reader); }
		} else {
			quote! { }
		};
		let val = if is_bool {
			quote! { reader.read_bit()? }
		} else {
			let parse = quote! { crate::world::GmParam::deserialize(reader)? };
			match default {
				None => quote! { #parse },
				Some(default) => quote! {
					if reader.read_bit()? {
						#parse
					} else {
						#default
					}
				},
			}
		};
		deser.push(quote! {
			#create_bitreader
			let #ident = #val;
		});
	}
	quote! {
		#(#deser)*
		Ok(Self {#(#idents)* })
	}
}

fn gen_ser_code(fields: &Fields) -> TokenStream {
	let fields = match fields {
		Fields::Named(fields) => fields,
		_ => unimplemented!(),
	};
	let mut msg_needs_bitwriter = false;
	let mut ser = vec![];
	for f in &fields.named {
		let ident = &f.ident;

		let is_bool = match &f.ty {
			Type::Path(path) => path.path.is_ident("bool"),
			_ => false,
		};
		let default = get_gm_default(&f);
		let field_needs_bitwriter = is_bool || default.is_some();
		let create_bitwriter = if !msg_needs_bitwriter && field_needs_bitwriter {
			msg_needs_bitwriter = true;
			quote! { let mut writer = &mut ::endio_bit::BEBitWriter::new(writer); }
		} else {
			quote! { }
		};
		let write = if is_bool {
			quote! { writer.write_bit(self.#ident)?; }
		} else {
			let write_field = quote! { crate::world::GmParam::serialize(&self.#ident, writer)? };
			match default {
				None => quote! { #write_field; },
				Some(default) => quote! {
					let is_not_default = self.#ident != #default;
					writer.write_bit(is_not_default)?;
					if is_not_default {
						#write_field;
					}
				},
			}
		};
		ser.push(quote! {
			#create_bitwriter
			#write
		});
	}
	quote! {
		#(#ser)*
		Ok(())
	}
}

fn get_gm_default(input: &Field) -> Option<NestedMeta> {
	for attr in &input.attrs {
		if !attr.path.is_ident("default") {
			continue;
		}
		let meta = match attr.parse_meta() {
			Err(_) => panic!("encountered unparseable default attribute"),
			Ok(x) => x,
		};
		let list = match meta {
			Meta::List(x) => x,
			_ => panic!("default attribute has wrong format"),
		};
		let nested_meta = list.nested.first().expect("default attribute should have exactly one argument");
		return Some(nested_meta.clone());
	}
	None
}
