use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Generics, Ident};

pub fn derive(input: proc_macro::TokenStream, opt_dest: Option<&Ident>) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let data = match &input.data {
		Data::Enum(data) => data,
		_ => panic!("only enums are supported"),
	};

	let name = &input.ident;
	let dest;
	let generics;
	if let Some(d) = opt_dest {
		dest = d;
		generics = Generics::default();
	} else {
		dest = name;
		generics = input.generics;
	}
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

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
			impl #impl_generics ::std::convert::From<#variant_ty> for #dest #ty_generics #where_clause {
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
