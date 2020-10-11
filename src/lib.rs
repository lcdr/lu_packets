/*!
	Documentation and (de-)serialization support for LU's network protocol.
*/
#![feature(arbitrary_enum_discriminant)]

/**
	Creates a [`LuNameValue`] containing the arguments.

	The syntax is `name: value`, where `name` is a string literal that will be converted to a [`LuVarWString<u32>`], and `value` is an expression that will be converted to an [`LnvValue`].

	Example:

	```
	lnv! {
		"wstring": "string expression",
		"i32": 42i32,
		"f32": 3.14f32,
		"f64": 3.14f64,
		"u32": 42u32,
		"bool": true,
		"i64": i64::MAX,
		"u64": u64::MAX,
		"string": b"byte slice"[..],
	}
	```

	Care should be taken with integer and float literals to suffix them with the correct type, as seen above. Rust assumes `i32` for integer and `f64` for float literals by default, which may not be what you want, and can lead to incorrect serialization.

	[`LuNameValue`]: crate::world::lnv::LuNameValue
	[`LuVarWString<u32>`]: crate::common::str::variable::LuVarWString
	[`LnvValue`]: crate::world::lnv::LnvValue
*/
#[macro_export]
macro_rules! lnv {
	{} => { crate::world::lnv::LuNameValue::new() };
	{$($name:literal:$value:expr,)*} => {
		{
			let mut lnv = crate::world::lnv::LuNameValue::new();
			$(lnv.insert(::std::convert::TryInto::try_into($name).unwrap(), $value.into());)*
			lnv
		}
	}
}

/**
	Converts the argument to a LU string.

	This forwards to the [`TryInto`] implementation on the argument, which means the macro is flexible and works with both string and wstring types, and both fixed and variable types, depending on context. Generally, to convert to a string type, pass a byte slice, and for a wstring type, pass a `&str` or `String`.

	[`TryInto`]: std::convert::TryInto
*/
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
