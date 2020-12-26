use std::io::{Error, ErrorKind::InvalidData, Read, Result as Res, Write};

use endio::{Deserialize, LE, LERead, LEWrite, Serialize};
use endio_bit::{BEBitReader, BEBitWriter};
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::common::{LuVarWString, ObjId};
use super::ComponentConstruction;

#[derive(Debug, PartialEq)]
pub enum TransitionState {
	None,
	Arrive { last_custom_build_parts: LuVarWString<u16> },
	Leave,
}

impl<R: Read> Deserialize<LE, BEBitReader<R>> for TransitionState {
	fn deserialize(reader: &mut BEBitReader<R>) -> Res<Self> {
		let disc = reader.read_bits(2)?;
		Ok(match disc {
			0 => TransitionState::None,
			1 => TransitionState::Arrive { last_custom_build_parts: LERead::read(reader)? },
			2 => TransitionState::Leave,
			_ => { return Err(Error::new(InvalidData, "invalid discriminant for TransitionState")) }
		})
	}
}

impl<'a, W: Write> Serialize<LE, BEBitWriter<W>> for &'a TransitionState {
	fn serialize(self, writer: &mut BEBitWriter<W>) -> Res<()> {
		match self {
			TransitionState::None => writer.write_bits(0, 2),
			TransitionState::Arrive { last_custom_build_parts } => { writer.write_bits(1, 2)?; LEWrite::write(writer, last_custom_build_parts) },
			TransitionState::Leave => writer.write_bits(2, 2),
		}
	}
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct GmPvpInfo {
	pub pvp_enabled: bool,
	pub is_gm: bool,
	pub gm_level: u8,
	pub editor_enabled: bool,
	pub editor_level: u8,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[repr(u32)]
pub enum GameActivity {
	None,
	Quickbuilding,
	ShootingGallery,
	Racing,
	Pinball,
	PetTaming,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
#[trailing_padding=4] // country code, unused
pub struct SocialInfo {
	pub guild_id: ObjId,
	pub guild_name: LuVarWString<u8>,
	pub is_lego_club_member: bool,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct CharacterConstruction {
	pub claim_code_1: Option<u64>,
	pub claim_code_2: Option<u64>,
	pub claim_code_3: Option<u64>,
	pub claim_code_4: Option<u64>,
	// todo: type for each of the below
	pub hair_color: u32,
	pub hair_style: u32,
	#[padding=4] // head style, unused
	pub torso_color: u32,
	pub legs_color: u32,
	pub torso_decal: u32,
	#[padding=4] // head color, unused
	pub eyebrows_style: u32,
	pub eyes_style: u32,
	pub mouth_style: u32,
	pub account_id: u64,
	pub last_logout: u64,
	pub prop_mod_last_display_time: u64,
	pub u_score: u64,
	pub is_free_trial: bool,
	pub total_currency_collected: u64,
	pub total_bricks_collected: u64,
	pub total_smashables_smashed: u64,
	pub total_quickbuilds_completed: u64,
	pub total_enemies_smashed: u64,
	pub total_rockets_used: u64,
	pub total_missions_completed: u64,
	pub total_pets_tamed: u64,
	pub total_imagination_powerups_collected: u64,
	pub total_life_powerups_collected: u64,
	pub total_armor_powerups_collected: u64,
	pub total_distance_traveled: u64,
	pub times_smashed_count: u64,
	pub total_damage_taken: u64,
	pub total_damage_healed: u64,
	pub total_armor_repaired: u64,
	pub total_imagination_restored: u64,
	pub total_imagination_used: u64,
	pub total_distance_driven: u64,
	pub total_time_airborne_in_a_race_car: u64,
	pub total_racing_imagination_powerups_collected: u64,
	pub total_racing_imagination_crates_smashed: u64,
	pub total_racing_car_boosts_activated: u64,
	pub total_racing_wrecks: u64,
	pub total_racing_smashables_smashed: u64,
	pub total_races_finished: u64,
	pub total_first_place_race_finishes: u64,
	pub transition_state: TransitionState,
	pub gm_pvp_info: Option<GmPvpInfo>,
	pub current_activity: Option<GameActivity>,
	pub social_info: Option<SocialInfo>,
}

impl ComponentConstruction for CharacterConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}
