use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, parse_quote, Attribute, Data, DataEnum, DeriveInput, Field, Fields, Generics, Lit, LitInt, Meta, NestedMeta};

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	let deser_code;
	let ser_code;
	match &input.data {
		Data::Struct(data) => {
			deser_code = gen_deser_code_struct(&data.fields);
			ser_code = gen_ser_code_struct(&data.fields, &name);
		},
		Data::Enum(data) => {
			let ty = get_enum_type(&input);
			let pre_disc_padding = get_pre_disc_padding(&input);
			let post_disc_padding = get_post_disc_padding(&input);
			deser_code = gen_deser_code_enum(data, &name, &ty, &pre_disc_padding, &post_disc_padding);
			ser_code = gen_ser_code_enum(data, &name, &ty, &pre_disc_padding, &post_disc_padding, &input.generics);
		}
		Data::Union(_) => unimplemented!(),
	}

	let trailing_padding = get_trailing_padding(&input);
	let read_padding = gen_read_padding(&trailing_padding);
	let write_padding = gen_write_padding(&trailing_padding);

	let (_, ty_generics, where_clause) = input.generics.split_for_impl();

	let des_impl_generics = &mut input.generics.clone();
	des_impl_generics.params.push(parse_quote!(__READER: ::std::io::Read));
	let (des_impl_generics, _, _) = des_impl_generics.split_for_impl();

	let ser_impl_generics = &mut input.generics.clone();
	ser_impl_generics.params.push(parse_quote!('__LIFETIME));
	ser_impl_generics.params.push(parse_quote!(__WRITER: ::std::io::Write));
	let (ser_impl_generics, _, _) = ser_impl_generics.split_for_impl();

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

fn gen_deser_code_fields(fields: &Fields) -> TokenStream {
	match fields {
		Fields::Named(fields) => {
			let mut deser = vec![];
			for f in &fields.named {
				let ident = &f.ident;
				let padding = get_field_padding(f);
				let read_padding = gen_read_padding(&padding);
				deser.push(quote! { #ident: {
					#read_padding
					crate::raknet::client::replica::ReplicaD::deserialize(reader)?
				}, });
			}
			quote! { { #(#deser)* } }
		}
		Fields::Unnamed(fields) => {
			let mut deser = vec![];
			for f in &fields.unnamed {
				let padding = get_field_padding(f);
				let read_padding = gen_read_padding(&padding);
				deser.push(quote! { {
					#read_padding
					crate::raknet::client::replica::ReplicaD::deserialize(reader)?
				}, });
			}
			quote! { ( #(#deser)* ) }
		}
		Fields::Unit => {
			quote! { }
		}
	}
}

fn gen_deser_code_struct(fields: &Fields) -> TokenStream {
	let deser_code = gen_deser_code_fields(fields);
	quote! { let ret = Self #deser_code; }
}

fn gen_deser_code_enum(data: &DataEnum, name: &Ident, ty: &Ident, pre_disc_padding: &Option<LitInt>, post_disc_padding: &Option<LitInt>) -> TokenStream {
	let last_disc: syn::ExprLit = parse_quote! { 0 };
	let mut last_disc = &last_disc.into();
	let mut disc_offset = 0;
	let mut arms = vec![];
	for f in &data.variants {
		let ident = &f.ident;
		if let Some((_, x)) = &f.discriminant {
			last_disc = x;
			disc_offset = 0;
		}
		let deser_fields = gen_deser_code_fields(&f.fields);
		let arm = quote! { disc if disc == (#last_disc + (#disc_offset as #ty)) => Self::#ident #deser_fields, };
		disc_offset += 1;
		arms.push(arm);
	}
	let read_pre_padding = gen_read_padding(pre_disc_padding);
	let read_post_padding = gen_read_padding(post_disc_padding);
	quote! {
		#read_pre_padding
		let disc: #ty = ::endio::LERead::read(reader)?;
		#read_post_padding
		let ret = match disc {
			#(#arms)*
			_ => return ::std::result::Result::Err(::std::io::Error::new(::std::io::ErrorKind::InvalidData, format!("invalid discriminant value for {}: {}", stringify!(#name), disc)))
		};
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

fn gen_ser_code_fields(fields: &Fields) -> TokenStream {
	match fields {
		Fields::Named(fields) => {
			let mut pat = vec![];
			let mut ser = vec![];
			for f in &fields.named {
				let ident = &f.ident;
				let padding = get_field_padding(f);
				let write_padding = gen_write_padding(&padding);
				pat.push(quote! { #ident, });
				ser.push(quote! {
					#write_padding
					crate::raknet::client::replica::ReplicaS::serialize(#ident, writer)?;
				});
			}
			quote! { { #(#pat)* } => { #(#ser)* } }
		}
		Fields::Unnamed(fields) => {
			let mut index = String::from("a");
			let mut pat = vec![];
			let mut ser = vec![];
			for f in &fields.unnamed {
				let ident = Ident::new(&index, Span::call_site());
				let padding = get_field_padding(f);
				let write_padding = gen_write_padding(&padding);
				pat.push(quote! { #ident, });
				ser.push(quote! {
					#write_padding
					crate::raknet::client::replica::ReplicaS::serialize(#ident, writer)?;
				});
				index += "a";
			}
			quote! { ( #(#pat)* ) => { #(#ser)* } }
		}
		Fields::Unit => {
			quote! { => {} }
		}
	}
}

fn gen_ser_code_struct(fields: &Fields, name: &Ident) -> TokenStream {
	let ser_code = gen_ser_code_fields(fields);
	quote! {
		match self {
			#name #ser_code
		}
	}
}

fn gen_ser_code_enum(data: &DataEnum, name: &Ident, ty: &Ident, pre_disc_padding: &Option<LitInt>, post_disc_padding: &Option<LitInt>, generics: &Generics) -> TokenStream {
	let mut arms = vec![];
	for f in &data.variants {
		let ident = &f.ident;
		let ser_fields = gen_ser_code_fields(&f.fields);
		let expanded = quote! { #name::#ident #ser_fields };
		arms.push(expanded);
	}
	let write_pre_padding = gen_write_padding(pre_disc_padding);
	let write_post_padding = gen_write_padding(post_disc_padding);
	quote! {
		#write_pre_padding
		let disc = unsafe { *(self as *const #name #generics as *const #ty) };
		::endio::LEWrite::write(writer, disc)?;
		#write_post_padding
		match self {
			#(#arms)*
		}
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

fn get_enum_type(input: &DeriveInput) -> Ident {
	for attr in &input.attrs {
		if !attr.path.is_ident("repr") {
			continue;
		}
		let meta = match attr.parse_meta() {
			Err(_) => panic!("encountered unparseable repr attribute"),
			Ok(x) => x,
		};
		let list = match meta {
			Meta::List(x) => x,
			_ => continue,
		};
		if list.nested.is_empty() {
			panic!("encountered repr attribute with no arguments");
		}
		for nested_meta in list.nested {
			let meta = match nested_meta {
				NestedMeta::Meta(x) => x,
				NestedMeta::Lit(_) => continue,
			};
			let path = match meta {
				Meta::Path(x) => x,
				_ => continue,
			};
			if path.is_ident("C") || path.is_ident("transparent") {
				continue;
			}
			return (*path.get_ident().expect("invalid repr attribute argument")).clone();
		}
	}
	panic!("You need to add a repr attribute to specify the discriminant type, e.g. #[repr(u16)]");
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

fn get_pre_disc_padding(input: &DeriveInput) -> Option<LitInt> {
	get_padding(&input.attrs, "pre_disc_padding")
}

fn get_post_disc_padding(input: &DeriveInput) -> Option<LitInt> {
	get_padding(&input.attrs, "post_disc_padding")
}

fn get_trailing_padding(input: &DeriveInput) -> Option<LitInt> {
	get_padding(&input.attrs, "trailing_padding")
}
