use std::collections::HashMap;
use std::convert::TryInto;

use endio::{LERead, LEWrite};
use super::gm::GmParam;

use crate::common::{LuStrExt, LuVarString, LuVarWString, LuWStr, Ucs2Char};

#[derive(Debug)]
#[repr(u8)]
enum LnvValue {
	WString(LuVarWString<u32>) = 0,
	I32(i32) = 1,
	F32(f32) = 3,
	F64(f64) = 4,
	U32(u32) = 5,
	Bool(bool) = 7,
	I64(i64) = 8,
	U64(u64) = 9,
	String(LuVarString<u32>) = 13,
}

impl From<&LuWStr> for LnvValue {
	fn from(wstr: &LuWStr) -> Self {
		let string: String = wstr.to_string();
		let (ty, val) = string.split_at(string.find(":").unwrap());
		let val = val.split_at(1).1;
		dbg!(&ty);
		dbg!(&val);
		match ty {
			"0" => LnvValue::WString(val.try_into().unwrap()),
			"1" => LnvValue::I32(val.parse().unwrap()),
			"3" => LnvValue::F32(val.parse().unwrap()),
			"4" => LnvValue::F64(val.parse().unwrap()),
			"5" => LnvValue::U32(val.parse().unwrap()),
			"7" => LnvValue::Bool(val == "1"),
			"8" => LnvValue::I64(val.parse().unwrap()),
			"9" => LnvValue::U64(val.parse().unwrap()),
			"13" => LnvValue::String(val.try_into().unwrap()),
			_ => panic!(),
		}
	}
}

#[derive(Debug)]
pub struct LuNameValue(HashMap<LuVarWString<u32>, LnvValue>);

impl From<&LuVarWString<u32>> for LuNameValue {
	fn from(wstr: &LuVarWString<u32>) -> Self {
		if wstr.is_empty() {
			return LuNameValue(HashMap::new())
		}
		let mut map = HashMap::new();
		for name_type_val in wstr.split(|c| *c == Ucs2Char::new(b'\n'.into())) {
			let (name, type_val) = name_type_val.split_at(name_type_val.iter().position(|c| *c == Ucs2Char::new(b'='.into())).unwrap());
			let name: LuVarWString<u32> = name.into();
			let type_val = type_val.split_at(1).1;
			let lnv_value: LnvValue = type_val.into();
			map.insert(name, lnv_value);
		}
		LuNameValue(map)
	}
}

impl From<&LuNameValue> for LuVarWString<u32> {
	fn from(lnv: &LuNameValue) -> Self {
		panic!()
	}
}

impl GmParam for LuNameValue {
	fn deserialize<R: ::std::io::Read>(reader: &mut R) -> ::std::io::Result<Self> {
		let lu_var_wstr: LuVarWString<u32> = LERead::read(reader)?;
		if !lu_var_wstr.is_empty() {
			let _: u16 = LERead::read(reader)?;
		}
		Ok((&lu_var_wstr).into())
	}

	fn serialize<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
		let lu_var_wstr: LuVarWString<u32> = self.into();
		LEWrite::write(writer, &lu_var_wstr)?;
		if !lu_var_wstr.is_empty() {
			LEWrite::write(writer, 0u16)?; // for some reason has a null terminator
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use std::convert::TryInto;

	use crate::common::LuVarWString;
	use super::LuNameValue;

	#[test]
	fn from_wstr() {
		let input = "SERVERNAME=0:Overbuild Universe (US)\nPATCHSERVERIP=0:localhost\nAUTHSERVERIP=0:localhost\nPATCHSERVERPORT=1:80\nLOGGING=1:100\nDATACENTERID=5:150\nCPCODE=1:89164\nAKAMAIDLM=7:0\nPATCHSERVERDIR=0:luclient\nUGCUSE3DSERVICES=7:1\nUGCSERVERIP=0:localhost\nUGCSERVERDIR=0:3dservices\nPASSURL=0:https://account.lego.com/en-us/SendPassword.aspx?Username=\nSIGNINURL=0:https://account.lego.com/en-us/SignIn.aspx?ReturnUrl=http://universe.lego.com/en-us/myaccount/default.aspx\nSIGNUPURL=0:http://universe.lego.com/en-us/myaccount/registration/default.aspx\nREGISTERURL=0:https://secure.universe.lego.com/en-us/myaccount/subscription/embeddedlandingpage.aspx?username=\nCRASHLOGURL=0:http://services.lego.com/cls.aspx\nLOCALE=0:en_US\nTRACK_DSK_USAGE=7:1\nMULTISAMPLING=1:5";
		let wstr: LuVarWString<u32> = input.try_into().unwrap();
		let lnv: LuNameValue = (&wstr).try_into().unwrap();
	}
}
