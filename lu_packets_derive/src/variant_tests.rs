use proc_macro2::{Span, TokenStream};
use syn::{parse_macro_input, Attribute, token::Comma, Data, DeriveInput, Ident, Meta, NestedMeta, punctuated::Punctuated};
use quote::quote;

// todo: only run this when generating tests
pub fn derive(input: proc_macro::TokenStream, reader_code: TokenStream, writer_code: TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let test_params = get_test_params(&input.attrs);

	let mut tests = vec![];

	match &input.data {
		Data::Struct(_) => {
			let test = gen_test_case(name, &test_params, name, &reader_code, &writer_code);
			tests.push(test);
		}
		Data::Enum(data) => {
			for v in &data.variants {
				let variant = &v.ident;
				let test = gen_test_case(name, &test_params, variant, &reader_code, &writer_code);
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

fn gen_test_case(type_name: &Ident, test_params: &Option<Punctuated<NestedMeta, Comma>>, test_name: &Ident, reader_code: &TokenStream, writer_code: &TokenStream) -> TokenStream {
	let bin_path = format!("tests/{}.bin", test_name);
	let rs_path = format!("tests/{}.rs", test_name);

	let test_name = Ident::new(&format!("_{}", test_name), Span::call_site());

	quote! {
		#[test]
		#[allow(non_snake_case)]
		fn #test_name() {
			use super::*;
			let mut bin = &include_bytes!(#bin_path)[..];
			let mut expected = include!(#rs_path);
			let mut input = bin;
			let mut reader = #reader_code;
			let parsed: #type_name<#test_params> = ::endio::LERead::read(&mut reader).expect("error while parsing bin");
			let mut read_buf = [0];
			let amount_read = std::io::Read::read(&mut reader, &mut read_buf).unwrap();
			assert_eq!(amount_read, 0, "bin not fully read");
			assert_eq!(parsed, expected, "parsed struct does not match rs");
			/*
			if parsed != expected {
				// bless input
				let mut out = vec![];
				{
					let mut writer = #writer_code;
					::endio::LEWrite::write(&mut writer, &expected).unwrap();
				}
				let mut out_filename = std::path::PathBuf::from(file!());
				out_filename.pop();
				out_filename.push(#bin_path);
				let mut file = std::fs::File::create(out_filename).unwrap();
				std::io::Write::write_all(&mut file, &mut out).unwrap();
				return;
			}
			*/
			let mut out = vec![];
			{
				let mut writer = #writer_code;
				::endio::LEWrite::write(&mut writer, &parsed).unwrap();
			}
			assert_eq!(out, input, "serialized struct does not match bin");
			/*
			if out != input {
				// bless output
				let mut out = std::fs::File::create().unwrap();
				::endio::LEWrite::write(&mut out, &parsed).unwrap();
			}
			*/
		}
	}
}
