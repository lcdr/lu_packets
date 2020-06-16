use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let impl_generics = &mut input.generics.clone();
	impl_generics.params.push(parse_quote!(__READER: ::std::io::Read));
	let (impl_generics,	_, _) = impl_generics.split_for_impl();
	let (_, ty_generics, where_clause) = input.generics.split_for_impl();

	(quote! {
		impl #impl_generics crate::world::GmDeserialize<__READER> for #name #ty_generics #where_clause {
			fn deserialize(reader: &mut __READER) -> ::std::io::Result<Self> {
				::endio::LERead::read(reader)
			}
		}
	}).into()
}
