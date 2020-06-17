mod from_variants;
mod game_message;
mod gm_deserialize;
mod service_message;

use proc_macro::TokenStream;

#[proc_macro_derive(FromVariants)]
pub fn derive_from_variants(input: TokenStream) -> TokenStream {
	from_variants::derive(input)
}

#[proc_macro_derive(GameMessage, attributes(default))]
pub fn derive_game_message_deserialize(input: TokenStream) -> TokenStream {
	game_message::derive(input)
}

#[proc_macro_derive(GmDeserialize)]
pub fn derive_gm_deserialize(input: TokenStream) -> TokenStream {
	gm_deserialize::derive(input)
}

#[proc_macro_derive(ServiceMessage)]
pub fn derive_service_message(input: TokenStream) -> TokenStream {
	service_message::derive(input)
}
