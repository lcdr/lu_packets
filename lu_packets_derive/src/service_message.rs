use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DataEnum, DeriveInput, Fields};

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let data = match &input.data {
		Data::Enum(data) => data,
		_ => unimplemented!(),
	};
	let name = &input.ident;
	let deser_code = gen_deser_code_enum(data, &name);
	let impl_generics = &mut input.generics.clone();
	impl_generics.params.push(parse_quote!(__READER: ::std::io::Read));
	let (impl_generics,	_, _) = impl_generics.split_for_impl();
	let (_, ty_generics, where_clause) = input.generics.split_for_impl();

	(quote! {
		impl #impl_generics ::endio::Deserialize<::endio::LE, __READER> for #name #ty_generics #where_clause {
			fn deserialize(reader: &mut __READER) -> ::std::io::Result<Self> {
				#deser_code
			}
		}
	}).into()
}

fn gen_deser_code_fields(fields: &Fields) -> TokenStream {
	match fields {
		Fields::Named(_) => unimplemented!(),
		Fields::Unnamed(fields) => {
			let mut deser = vec![];
			for _ in &fields.unnamed {
				deser.push(quote! { ::endio::LERead::read(reader)?, });
			}
			quote! { ( #(#deser)* ) }
		}
		Fields::Unit => {
			quote! { }
		}
	}
}

fn gen_deser_code_enum(data: &DataEnum, name: &Ident) -> TokenStream {
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
		let arm = quote! { disc if disc == (#last_disc + (#disc_offset as u32)) => Self::#ident #deser_fields, };
		disc_offset += 1;
		arms.push(arm);
	}
	quote! {
		let disc: u32 = ::endio::LERead::read(reader)?;
		let _padding: u8 = ::endio::LERead::read(reader)?;
		Ok(match disc {
			#(#arms)*
			_ => return ::std::result::Result::Err(::std::io::Error::new(::std::io::ErrorKind::InvalidData, format!("invalid discriminant value for {}: {}", stringify!(#name), disc)))
		})
	}
}
