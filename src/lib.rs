/*!
	Documentation and (de-)serialization support for LU's network protocol.
*/
#![feature(arbitrary_enum_discriminant)]
#![feature(min_const_generics)]
#![feature(specialization)]
#![allow(incomplete_features)]

/**
	Creates an [`Amf3::Array`](crate::world::amf3::Amf3::Array) containing the arguments.

	Since AMF3 arrays are both vectors and maps at the same time, there are multiple forms of the macro, for map and for vector usage.

	### Map usage

	The syntax is `name: value`, where `name` is a string literal that will be converted to an [`Amf3String`](crate::world::amf3::Amf3String), and `value` is an expression that will be converted to an [`Amf3`] object.

	Example:

	```
	# #[macro_use] extern crate lu_packets;
	# use lu_packets::amf3;
	# fn main() {
	amf3! {
		"false": false,
		"true": true,
		"double1": 3.14f32,
		"double2": 3.14f64,
		"string": "string",
		"array": amf3! { "inner": "array"},
	};
	# }
	```

	### Vector usage

	The syntax is the exact same as with the [`vec!`] macro, except that the arguments will be converted to an [`Amf3`] object before being inserted.

	Example:

	```
	# #[macro_use] extern crate lu_packets;
	# use lu_packets::amf3;
	# fn main() {
	amf3! [true, false, true];
	amf3! [true; 4];
	# }
	```

	[`Amf3`]: crate::world::amf3::Amf3
*/
#[macro_export]
macro_rules! amf3 {
	{} => { $crate::world::amf3::Amf3::Array($crate::world::amf3::Amf3Array::new()) };
	($($name:literal:$value:expr),+ $(,)?) => {
		{
			let mut array = $crate::world::amf3::Amf3Array::new();
			$(array.map.insert(::std::convert::TryInto::try_into($name).unwrap(), ::std::convert::TryInto::try_into($value).unwrap());)*
			$crate::world::amf3::Amf3::Array(array)
		}
	};
	($value:expr; $n:expr) => {
		{
			let converted = ::std::convert::TryInto::try_into($value).unwrap();
			let mut array = $crate::world::amf3::Amf3Array {
				map: ::std::collections::HashMap::new(),
				vec: vec![converted; $n],
			};
			$crate::world::amf3::Amf3::Array(array)
		}
	};
	($($value:expr),+ $(,)?) => {
		{
			let mut array = $crate::world::amf3::Amf3Array::new();
			$(array.vec.push(::std::convert::TryInto::try_into($value).unwrap());)*
			$crate::world::amf3::Amf3::Array(array)
		}
	};
}

/**
	Creates a [`LuNameValue`] containing the arguments.

	The syntax is `name: value`, where `name` is a string literal that will be converted to a [`LuVarWString<u32>`], and `value` is an expression that will be converted to an [`LnvValue`].

	Example:

	```
	# #[macro_use] extern crate lu_packets;
	# use lu_packets::lnv;
	# fn main() {
	lnv! {
		"wstring": "string expression",
		"i32": 42i32,
		"f32": 3.14f32,
		"f64": 3.14f64,
		"u32": 42u32,
		"bool": true,
		"i64": i64::MAX,
		"u64": u64::MAX,
		"string": b"byte slice",
	};
	# }
	```

	Care should be taken with integer and float literals to suffix them with the correct type, as seen above. Rust assumes `i32` for integer and `f64` for float literals by default, which may not be what you want, and can lead to incorrect serialization.

	[`LuNameValue`]: crate::world::LuNameValue
	[`LuVarWString<u32>`]: crate::common::LuVarWString
	[`LnvValue`]: crate::world::LnvValue
*/
#[macro_export]
macro_rules! lnv {
	{} => { $crate::world::LuNameValue::new() };
	{$($name:literal:$value:expr,)*} => {
		{
			let mut lnv = $crate::world::LuNameValue::new();
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
