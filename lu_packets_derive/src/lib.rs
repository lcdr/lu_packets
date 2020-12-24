mod from_variants;
mod game_message;
mod gm_type;
mod replica_serde;
mod variant_tests;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(BitVariantTests, attributes(test_params))]
pub fn derive_bit_variant_tests(input: TokenStream) -> TokenStream {
	variant_tests::derive(input, quote!(::endio_bit::BEBitReader::new(&mut bin)), quote!(::endio_bit::BEBitWriter::new(&mut out)))
}

#[proc_macro_derive(FromVariants)]
pub fn derive_from_variants(input: TokenStream) -> TokenStream {
	from_variants::derive(input)
}

#[proc_macro_derive(GameMessage, attributes(default))]
pub fn derive_game_message_deserialize(input: TokenStream) -> TokenStream {
	game_message::derive(input)
}

#[proc_macro_derive(GmParam)]
pub fn derive_gm_type(input: TokenStream) -> TokenStream {
	gm_type::derive(input)
}

#[proc_macro_derive(ReplicaSerde, attributes(padding, trailing_padding))]
pub fn derive_replica(input: TokenStream) -> TokenStream {
	replica_serde::derive(input)
}

#[proc_macro_derive(ReplicaVariantTests, attributes(test_params))]
pub fn derive_replica_variant_tests(input: TokenStream) -> TokenStream {
	variant_tests::derive(input, quote!(crate::raknet::client::replica::DummyContext { inner: &mut bin }), quote!(&mut out))
}

#[proc_macro_derive(VariantTests, attributes(test_params))]
pub fn derive_variant_tests(input: TokenStream) -> TokenStream {
	variant_tests::derive(input, quote!(bin), quote!(&mut out))
}
