#![feature(arbitrary_enum_discriminant)]

#[macro_export]
macro_rules! lnv {
	{} => { crate::world::lnv::LuNameValue::new() };
	{$($name:literal:$val:expr,)*} => {
		{
			let mut lnv = crate::world::lnv::LuNameValue::new();
			$(lnv.insert(::std::convert::TryInto::try_into($name).unwrap(), $val.into());)*
			lnv
		}
	}
}

#[macro_export]
macro_rules! lu {
	($str_lit:expr) => {
		::std::convert::TryInto::try_into($str_lit).unwrap()
	}
}

pub mod raknet;
pub mod auth;
pub mod chat;
pub mod common;
pub mod general;
pub mod world;
