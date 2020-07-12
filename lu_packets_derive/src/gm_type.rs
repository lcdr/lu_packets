use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	(quote! {
		impl #impl_generics crate::world::gm::GmParam for #name #ty_generics #where_clause {
			fn deserialize<R: ::std::io::Read>(reader: &mut R) -> ::std::io::Result<Self> {
				::endio::LERead::read(reader)
			}

			fn serialize<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
				::endio::LEWrite::write(writer, self)
			}
		}
	}).into()
}
