use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DataEnum, DeriveInput, Fields, Generics};

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let data = match &input.data {
		Data::Enum(data) => data,
		_ => unimplemented!(),
	};
	let name = &input.ident;

	let ser_code = gen_ser_code_enum(data, &name, &input.generics);
	let ser_impl = &mut input.generics.clone();
	ser_impl.params.push(parse_quote!('__LIFETIME));
	ser_impl.params.push(parse_quote!(__WRITER: ::std::io::Write));
	let (ser_impl,	_, _) = ser_impl.split_for_impl();

	let (_, ty_generics, where_clause) = input.generics.split_for_impl();

	(quote! {
		impl #ser_impl ::endio::Serialize<::endio::LE, __WRITER> for &'__LIFETIME #name #ty_generics #where_clause {
			fn serialize(self, writer: &mut __WRITER) -> ::std::io::Result<()> {
				#ser_code
			}
		}
	}).into()
}

fn gen_ser_code_fields(fields: &Fields) -> TokenStream {
	match fields {
		Fields::Named(_) => unimplemented!(),
		Fields::Unnamed(fields) => {
			let mut index = String::from("a");
			let mut pat = vec![];
			let mut ser = vec![];
			for _ in &fields.unnamed {
				let ident = Ident::new(&index, Span::call_site());
				pat.push(quote! { #ident, });
				ser.push(quote! { ::endio::LEWrite::write(writer, #ident)?; });
				index += "a";
			}
			quote! { ( #(#pat)* ) => { #(#ser)* } }
		}
		Fields::Unit => {
			quote! { => {} }
		}
	}
}

fn gen_ser_code_enum(data: &DataEnum, name: &Ident, generics: &Generics) -> TokenStream {
	let mut arms = vec![];
	for f in &data.variants {
		let ident = &f.ident;
		let ser_fields = gen_ser_code_fields(&f.fields);
		let expanded = quote! { #name::#ident #ser_fields };
		arms.push(expanded);
	}
	quote! {
		let disc = unsafe { *(self as *const #name #generics as *const u32) };
		::endio::LEWrite::write(writer, disc)?;
		::endio::LEWrite::write(writer, 0u8)?;
		match self {
			#(#arms)*
		}
		Ok(())
	}
}
