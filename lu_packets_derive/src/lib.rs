use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(FromVariants)]
pub fn derive_deserialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let data = match &input.data {
		Data::Enum(data) => data,
		_ => panic!("only enums are supported"),
	};

	let name = &input.ident;

	let mut impls = vec![];
	for v in &data.variants {
		let variant = &v.ident;
		let fields = match &v.fields {
			Fields::Named(_) => panic!("use a tuple or unit variant"),
			Fields::Unit => { continue }
			Fields::Unnamed(fields) => fields,
		};

		if fields.unnamed.len() != 1 {
			panic!("use exactly one tuple argument");
		}
		let first = fields.unnamed.first().unwrap();
		let variant_ty = &first.ty;

		let impl_ = quote! {
			impl ::std::convert::From<#variant_ty> for Message {
				fn from(msg: #variant_ty) -> Self {
					#name::#variant(msg).into()
				}
			}
		};
		impls.push(impl_);
	}
	(quote! {
		#(#impls)*
	}).into()
}
