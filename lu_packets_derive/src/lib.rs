mod from_variants;
mod game_message;
mod gm_type;
mod service_message_d;
mod service_message_s;

use proc_macro::TokenStream;

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

#[proc_macro_derive(ServiceMessageD)]
pub fn derive_service_message_d(input: TokenStream) -> TokenStream {
	service_message_d::derive(input)
}

#[proc_macro_derive(ServiceMessageS)]
pub fn derive_service_message_s(input: TokenStream) -> TokenStream {
	service_message_s::derive(input)
}
