use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::common::{LuVarWString, ObjId};
use super::{ComponentConstruction, ComponentSerialization};

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct ModuleAssemblyInfo {
	pub assembly_id: Option<ObjId>,
	pub use_optional_parts: bool,
	pub blob: LuVarWString<u16>,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct ModuleAssemblyConstruction {
	pub module_assembly_info: Option<ModuleAssemblyInfo>,
}

impl ComponentConstruction for ModuleAssemblyConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}

pub type ModuleAssemblySerialization = ModuleAssemblyConstruction;

impl ComponentSerialization for ModuleAssemblySerialization {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}
