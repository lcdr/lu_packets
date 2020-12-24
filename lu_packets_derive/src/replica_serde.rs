use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Attribute, Data, DeriveInput, Field, Fields, Lit, LitInt, Meta};

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

	let trailing_padding = get_trailing_padding(&input);
	let read_padding = gen_read_padding(&trailing_padding);
	let write_padding = gen_write_padding(&trailing_padding);

	let (_, ty_generics, where_clause) = input.generics.split_for_impl();

	let data = match &input.data {
		Data::Struct(data) => data,
		_ => unimplemented!(),
	};

	let deser_code = gen_deser_code(&data.fields);
	let ser_code = gen_ser_code(&data.fields);
	(quote! {
		impl #des_impl_generics ::endio::Deserialize<::endio::LE, ::endio_bit::BEBitReader<__READER>> for #name #ty_generics #where_clause {
			fn deserialize(reader: &mut ::endio_bit::BEBitReader<__READER>) -> ::std::io::Result<Self> {
				#deser_code
				#read_padding
				Ok(ret)
			}
		}

		impl #ser_impl_generics ::endio::Serialize<::endio::LE, ::endio_bit::BEBitWriter<__WRITER>> for &'__LIFETIME #name #ty_generics #where_clause {
			fn serialize(self, writer: &mut ::endio_bit::BEBitWriter<__WRITER>) -> ::std::io::Result<()> {
				#ser_code
				#write_padding
				Ok(())
			}
		}
	}).into()
}

fn gen_deser_code(fields: &Fields) -> TokenStream {
	let fields = match fields {
		Fields::Named(fields) => fields,
		_ => unimplemented!(),
	};
	let mut deser = vec![];
	for f in &fields.named {
		let ident = &f.ident;
		let padding = get_field_padding(f);
		let read_padding = gen_read_padding(&padding);
		deser.push(quote! { #ident: {
			#read_padding
			crate::raknet::client::replica::ReplicaD::deserialize(reader)?
		},
		});
	}
	quote! {
		let ret = Self {#(#deser)* };
	}
}

fn gen_read_padding(padding: &Option<LitInt>) -> TokenStream {
	match padding {
		Some(x) => quote! {
			let mut padding = [0; #x];
			::std::io::Read::read_exact(reader, &mut padding)?;
		},
		None => quote! { },
	}
}

fn gen_ser_code(fields: &Fields) -> TokenStream {
	let fields = match fields {
		Fields::Named(fields) => fields,
		_ => unimplemented!(),
	};
	let mut ser = vec![];
	for f in &fields.named {
		let ident = &f.ident;
		let padding = get_field_padding(f);
		let write_padding = gen_write_padding(&padding);
		let write = quote! {
			#write_padding
			crate::raknet::client::replica::ReplicaS::serialize(&self.#ident, writer)?;
		};
		ser.push(write);
	}
	quote! {
		#(#ser)*
	}
}

fn gen_write_padding(padding: &Option<LitInt>) -> TokenStream {
	match padding {
		Some(x) => quote! {
			let mut padding = [0; #x];
			::std::io::Write::write_all(writer, &padding)?;
		},
		None => quote! { },
	}
}


fn get_padding(attrs: &Vec<Attribute>, attr_name: &str) -> Option<LitInt> {
	for attr in attrs {
		if !attr.path.is_ident(attr_name) {
			continue;
		}
		let meta = match attr.parse_meta() {
			Err(_) => panic!("encountered unparseable {} attribute", attr_name),
			Ok(x) => x,
		};
		let lit = match meta {
			Meta::NameValue(x) => x.lit,
			_ => panic!("{} needs to be name=value", attr_name),
		};
		let int_lit = match lit {
			Lit::Int(x) => x,
			_ => panic!("{} needs to be an integer", attr_name),
		};
		return Some(int_lit);
	}
	None
}

fn get_field_padding(input: &Field) -> Option<LitInt> {
	get_padding(&input.attrs, "padding")
}

fn get_trailing_padding(input: &DeriveInput) -> Option<LitInt> {
	get_padding(&input.attrs, "trailing_padding")
}
