use proc_macro2::{Span, TokenStream};
use syn::{parse_macro_input, Attribute, token::Comma, Data, DeriveInput, Ident, Meta, NestedMeta, punctuated::Punctuated};
use quote::quote;

// todo: only run this when generating tests
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let test_params = get_test_params(&input.attrs);

	let mut tests = vec![];

	match &input.data {
		Data::Struct(_) => {
			let test = gen_test_case(name, &test_params, name);
			tests.push(test);
		}
		Data::Enum(data) => {
			for v in &data.variants {
				let variant = &v.ident;
				let test = gen_test_case(name, &test_params, variant);
				tests.push(test);
			}
		}
		Data::Union(_) => unimplemented!(),
	};

	let mod_name = Ident::new(&format!("_{}", name), Span::call_site());

	(quote! {
		#[cfg(test)]
		mod #mod_name {
			#(#tests)*
		}
	}).into()
}

fn get_test_params(attrs: &Vec<Attribute>) -> Option<Punctuated<NestedMeta, Comma>> {
	for attr in attrs {
		if !attr.path.is_ident("test_params") {
			continue;
		}
		let meta = match attr.parse_meta() {
			Err(_) => panic!("encountered unparseable test_params attribute"),
			Ok(x) => x,
		};
		let list = match meta {
			Meta::List(x) => x.nested,
			_ => panic!("test_params needs to be list"),
		};
		return Some(list);
	}
	None
}

fn gen_test_case(type_name: &Ident, test_params: &Option<Punctuated<NestedMeta, Comma>>, test_name: &Ident) -> TokenStream {
	let bin_path = format!("tests/{}.bin", test_name);
	let rs_path = format!("tests/{}.rs", test_name);

	let test_name = Ident::new(&format!("_{}", test_name), Span::call_site());

	quote! {
		#[test]
		#[allow(non_snake_case)]
		fn #test_name() {
			use super::*;
			let mut bin = &include_bytes!(#bin_path)[..];
			let mut val = include!(#rs_path);
			let mut input = bin;
			let mut reader = &mut bin;
			let parsed: #type_name<#test_params> = ::endio::LERead::read(reader).unwrap();
			assert_eq!(reader, &[]);
			assert_eq!(parsed, val);
			let mut out = vec![];
			::endio::LEWrite::write(&mut out, &parsed).unwrap();
			assert_eq!(out, input);
		}
	}
}
