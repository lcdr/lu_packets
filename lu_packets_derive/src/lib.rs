mod from_variants;
mod game_message;
mod gm_type;
mod service_message_d;
mod service_message_s;

use proc_macro::TokenStream;
use syn::{DeriveInput, Lit, LitInt, Meta};

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

#[proc_macro_derive(ServiceMessageD, attributes(disc_padding))]
pub fn derive_service_message_d(input: TokenStream) -> TokenStream {
	service_message_d::derive(input)
}

#[proc_macro_derive(ServiceMessageS, attributes(disc_padding))]
pub fn derive_service_message_s(input: TokenStream) -> TokenStream {
	service_message_s::derive(input)
}

fn get_disc_padding(input: &DeriveInput) -> Option<LitInt> {
	for attr in &input.attrs {
		if !attr.path.is_ident("disc_padding") {
			continue;
		}
		let meta = match attr.parse_meta() {
			Err(_) => panic!("encountered unparseable disc_padding attribute"),
			Ok(x) => x,
		};
		let lit = match meta {
			Meta::NameValue(x) => x.lit,
			_ => panic!("disc_padding needs to be name=value"),
		};
		let int_lit = match lit {
			Lit::Int(x) => x,
			_ => panic!("disc_padding needs to be an integer"),
		};
		return Some(int_lit);
	}
	None
}
