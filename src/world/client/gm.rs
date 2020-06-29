use std::cmp::PartialEq;

use endio::{Deserialize, Serialize};
use lu_packets_derive::{GameMessage, GmParam};

use crate::common::{ObjId, OBJID_EMPTY};

use super::super::{CloneId, CLONE_ID_INVALID, GmString, GmWString, InventoryType, KillType, Lot, LOT_NULL, MapId, MAP_ID_INVALID, MissionState, Quaternion, Vector3, ZoneId};
use super::super::gm::{EquipInventory, UnEquipInventory, MoveInventoryBatch};

#[derive(Debug, Deserialize, Serialize)]
pub struct SubjectGameMessage {
	pub subject_id: ObjId,
	pub message: GameMessage,
}

#[derive(Debug, Deserialize, Serialize)]
#[repr(u16)]
pub enum GameMessage {
	Teleport(Teleport) = 19,
	DropClientLoot(DropClientLoot) = 30,
	Die(Die) = 37,
	PlayAnimation(PlayAnimation) = 43,
	SetCurrency(SetCurrency) = 133,
	TeamPickupItem(TeamPickupItem) = 140,
	PlayFxEffect(PlayFxEffect) = 154,
	StopFxEffect(StopFxEffect) = 155,
	Resurrect(Resurrect) = 160,
	SetStunned(SetStunned) = 198,
	SetStunImmunity(SetStunImmunity) = 200,
	Knockback(Knockback) = 202,
	EnableRebuild(EnableRebuild) = 213,
	EquipInventory(EquipInventory) = 231,
	UnEquipInventory(UnEquipInventory) = 233,
	OfferMission(OfferMission) = 248,
	NotifyMission(NotifyMission) = 254,
	RebuildNotifyState(RebuildNotifyState) = 336,
	TerminateInteraction(TerminateInteraction) = 357,
	VendorOpenWindow = 369,
	EmotePlayed(EmotePlayed) = 371,
	SetInventorySize(SetInventorySize) = 389,
	ActivityStart = 407,
	SetUserCtrlCompPause(SetUserCtrlCompPause) = 466,
	NotifyClientFlagChange(NotifyClientFlagChange) = 472,
	Help(Help) = 475,
	VendorTransactionResult(VendorTransactionResult) = 476,
	HasBeenCollectedByClient(HasBeenCollectedByClient) = 487,
	PlayerReady = 509,
	TransferToZone(TransferToZone) = 516,
	TransferToZoneCheckedIm(TransferToZoneCheckedIm) = 517,
	InvalidZoneTransferList(InvalidZoneTransferList) = 519,
	TransferToLastNonInstance(TransferToLastNonInstance) = 527,
	DisplayMessageBox(DisplayMessageBox) = 529,
	SetJetPackMode(SetJetPackMode) = 561,
	UseItemResult(UseItemResult) = 607,
	SetEmoteLockState(SetEmoteLockState) = 693,
	UseItemRequirementsResponse(UseItemRequirementsResponse) = 703,
	PlayEmbeddedEffectOnAllClientsNearObject(PlayEmbeddedEffectOnAllClientsNearObject) = 713,
	NotifyClientZoneObject(NotifyClientZoneObject) = 737,
	PropertyRentalResponse(PropertyRentalResponse) = 750,
	PlatformResync(PlatformResync) = 761,
	PlayCinematic(PlayCinematic) = 762,
	OpenPropertyVendor = 861,
	ServerTradeInvite(ServerTradeInvite) = 870,
	ServerTradeInitialReply(ServerTradeInitialReply) = 873,
	ServerTradeFinalReply(ServerTradeFinalReply) = 874,
	ServerTradeAccept(ServerTradeAccept) = 884,
	RequestClientBounce(RequestClientBounce) = 934,
	MoveInventoryBatch(MoveInventoryBatch) = 957,
	ObjectActivatedClient(ObjectActivatedClient) = 980,
	NotifyClientObject(NotifyClientObject) = 1042,
	ModifyPlayerZoneStatistic(ModifyPlayerZoneStatistic) = 1046,
	StartArrangingWithItem(StartArrangingWithItem) = 1061,
	FinishArrangingWithItem(FinishArrangingWithItem) = 1062,
	SetBuildModeConfirmed(SetBuildModeConfirmed) = 1073,
	NotifyClientFailedPrecondition(NotifyClientFailedPrecondition) = 1081,
	EchoSyncSkill(EchoSyncSkill) = 1144,
	DoClientProjectileImpact(DoClientProjectileImpact) = 1151,
	SetPlayerAllowedRespawn(SetPlayerAllowedRespawn) = 1165,
	UncastSkill(UncastSkill) = 1206,
	FireEventClientSide(FireEventClientSide) = 1213,
	ChangeObjectWorldState(ChangeObjectWorldState) = 1223,
	LockNodeRotation(LockNodeRotation) = 1260,
	PlayerReachedRespawnCheckpoint(PlayerReachedRespawnCheckpoint) = 1296,
	MatchResponse(MatchResponse) = 1309,
	SetStatusImmunity(SetStatusImmunity) = 1435,
	CancelSkillCast = 1451,
	ModifyLegoScore(ModifyLegoScore) = 1459,
	RestoreToPostLoadStats = 1468,
	SetRailMovement(SetRailMovement) = 1471,
	StartRailMovement(StartRailMovement) = 1472,
	NotifyRailActivatorStateChange(NotifyRailActivatorStateChange) = 1478,
	UpdatePlayerStatistic(UpdatePlayerStatistic) = 1481,
	NotifyNotEnoughInvSpace(NotifyNotEnoughInvSpace) = 1516,
	NotifyPropertyOfEditMode(NotifyPropertyOfEditMode) = 1546,
	PropertyEntranceBegin = 1553,
	TeamAddPlayer(TeamAddPlayer) = 1562,
	TeamRemovePlayer(TeamRemovePlayer) = 1563,
	UpdatePropertyModelCount(UpdatePropertyModelCount) = 1595,
	StartCelebrationEffect(StartCelebrationEffect) = 1618,
	SetLocalTeam(SetLocalTeam) = 1636,
	ServerDoneLoadingAllObjects = 1642,
	PlayerSetCameraCyclingMode(PlayerSetCameraCyclingMode) = 1676,
	SetMountInventoryId(SetMountInventoryId) = 1726,
	NotifyLevelRewards(NotifyLevelRewards) = 1735,
	ClientCancelMoveSkill = 1747,
}

#[derive(Debug, GameMessage)]
pub struct Teleport {
	#[default(true)]
	pub ignore_y: bool,
	#[default(false)]
	pub set_rotation: bool,
	#[default(false)]
	pub skip_all_checks: bool,
	pub pos: Vector3,
	#[default(false)]
	pub use_navmesh: bool,
	#[default(1.0)]
	pub w: f32,
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[derive(Debug, GameMessage)]
pub struct DropClientLoot {
	#[default(false)]
	pub use_position: bool,
	#[default(Vector3::ZERO)]
	pub final_position: Vector3,
	pub currency: i32,
	pub item_template: Lot,
	pub loot_id: ObjId,
	pub owner: ObjId,
	pub source_obj: ObjId,
	#[default(Vector3::ZERO)]
	pub spawn_position: Vector3,
}

#[derive(Debug, GameMessage)]
pub struct Die {
	#[default(false)]
	pub client_death: bool,
	#[default(true)]
	pub spawn_loot: bool,
	pub death_type: GmWString,
	pub direction_relative_angle_xz: f32,
	pub direction_relative_angle_y: f32,
	pub direction_relative_force: f32,
	#[default(KillType::Violent)]
	pub kill_type: KillType,
	pub killer_id: ObjId,
	#[default(OBJID_EMPTY)]
	pub loot_owner_id: ObjId,
}

#[derive(Debug, GameMessage)]
pub struct PlayAnimation {
	pub animation_id: GmWString,
	#[default(true)]
	pub expect_anim_to_exist: bool,
	pub play_immediate: bool,
	#[default(false)]
	pub trigger_on_complete_msg: bool,
	#[default(SECONDARY_PRIORITY)]
	pub priority: f32,
	#[default(1.0)]
	pub scale: f32,
}

const SECONDARY_PRIORITY: f32 = 0.4;

#[derive(Debug, GameMessage)]
pub struct SetCurrency {
	pub currency: i64,
	#[default(LootType::None)]
	pub loot_type: LootType,
	pub position: Vector3,
	#[default(LOT_NULL)]
	pub source_lot: Lot,
	#[default(OBJID_EMPTY)]
	pub source_object: ObjId,
	#[default(OBJID_EMPTY)]
	pub source_trade_id: ObjId,
	#[default(LootType::None)]
	pub source_type: LootType,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum LootType {
	None,
	Chest,
	Mission,
	Mail,
	Currency,
	Achievement,
	Trade,
	Quickbuild,
	Deletion,
	Vendor,
	Activity,
	Pickup,
	Brick,
	Property,
	Moderation,
	Exhibit,
	Inventory,
	Claimcode,
	Consumption,
	Crafting,
	LevelReward,
	Relocate,
}

#[derive(Debug, GameMessage)]
pub struct TeamPickupItem {
	pub loot_id: ObjId,
	pub loot_owner_id: ObjId,
}

#[derive(Debug, GameMessage)]
pub struct PlayFxEffect {
	#[default(-1)]
	pub effect_id: i32,
	pub effect_type: GmWString,
	#[default(1.0)]
	pub scale: f32,
	pub name: GmString,
	#[default(1.0)]
	pub priority: f32,
	#[default(OBJID_EMPTY)]
	pub secondary: ObjId,
	#[default(true)]
	pub serialize: bool,
}

#[derive(Debug, GameMessage)]
pub struct StopFxEffect {
	pub kill_immediate: bool,
	pub name: GmString,
}

#[derive(Debug, GameMessage)]
pub struct Resurrect {
	#[default(false)]
	pub rez_immediately: bool,
}

#[derive(Debug, GameMessage)]
pub struct SetStunned {
	#[default(OBJID_EMPTY)]
	pub originator: ObjId,
	pub state_change_type: StunState,
	pub cant_attack: bool,
	#[default(false)]
	pub cant_attack_out_change_was_applied: bool,
	pub cant_equip: bool,
	#[default(false)]
	pub cant_equip_out_change_was_applied: bool,
	pub cant_interact: bool,
	#[default(false)]
	pub cant_interact_out_change_was_applied: bool,
	pub cant_jump: bool,
	#[default(false)]
	pub cant_jump_out_change_was_applied: bool,
	pub cant_move: bool,
	#[default(false)]
	pub cant_move_out_change_was_applied: bool,
	pub cant_turn: bool,
	#[default(false)]
	pub cant_turn_out_change_was_applied: bool,
	#[default(false)]
	pub cant_use_item: bool,
	#[default(false)]
	pub cant_use_item_out_change_was_applied: bool,
	#[default(false)]
	pub dont_terminate_interact: bool,
	#[default(true)]
	pub ignore_immunity: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum StunState {
	Push,
	Pop,
}

#[derive(Debug, GameMessage)]
pub struct SetStunImmunity {
	#[default(OBJID_EMPTY)]
	pub caster: ObjId,
	pub state_change_type: ImmunityState,
	pub immune_to_stun_attack: bool,
	pub immune_to_stun_equip: bool,
	pub immune_to_stun_interact: bool,
	pub immune_to_stun_jump: bool,
	pub immune_to_stun_move: bool,
	pub immune_to_stun_turn: bool,
	pub immune_to_stun_use_item: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum ImmunityState {
	Push,
	Pop,
}

#[derive(Debug, GameMessage)]
pub struct Knockback {
	#[default(OBJID_EMPTY)]
	pub caster: ObjId,
	#[default(OBJID_EMPTY)]
	pub originator: ObjId,
	#[default(0)]
	pub knock_back_time_ms: i32,
	pub vector: Vector3,
}

#[derive(Debug, GameMessage)]
pub struct EnableRebuild {
	pub enable: bool,
	pub fail: bool,
	pub success: bool,
	#[default(FailReason::NotGiven)]
	pub fail_reason: FailReason,
	pub duration: f32,
	pub user: ObjId,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum FailReason {
	NotGiven,
	OutOfImagination,
	CanceledEarly,
	BuildEnded,
}

#[derive(Debug, GameMessage)]
pub struct OfferMission {
	pub mission_id: i32,
	pub offerer: ObjId,
}

#[derive(Debug, GameMessage)]
pub struct NotifyMission {
	pub mission_id: i32,
	pub mission_state: MissionState,
	#[default(false)]
	pub sending_rewards: bool,
}

#[derive(Debug, GameMessage)]
pub struct RebuildNotifyState {
	pub prev_state: RebuildChallengeState,
	pub state: RebuildChallengeState,
	pub player: ObjId,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum RebuildChallengeState {
	Open = 0,
	Completed = 2,
	Resetting = 4,
	Building = 5,
	Incomplete = 6,
}

#[derive(Debug, GameMessage)]
pub struct TerminateInteraction {
	pub terminator_id: ObjId,
	pub terminate_type: TerminateType,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum TerminateType {
	Range,
	User,
	FromInteraction,
}

#[derive(Debug, GameMessage)]
pub struct EmotePlayed {
	pub emote_id: i32,
	pub target_id: ObjId,
}

#[derive(Debug, GameMessage)]
pub struct SetInventorySize {
	pub inventory_type: i32, // todo: type
	pub size: i32, // todo: check if can be made unsigned
}

#[derive(Debug, GameMessage)]
pub struct SetUserCtrlCompPause {
	pub paused: bool,
}

#[derive(Debug, GameMessage)]
pub struct NotifyClientFlagChange {
	pub flag: bool,
	pub flag_id: i32,
}

#[derive(Debug, GameMessage)]
pub struct Help {
	pub help_id: i32, // todo: type
}

#[derive(Debug, GameMessage)]
pub struct VendorTransactionResult {
	pub result: i32, // todo: type
}

#[derive(Debug, GameMessage)]
pub struct HasBeenCollectedByClient {
	pub player_id: ObjId,
}

#[derive(Debug, GameMessage)]
pub struct TransferToZone {
	#[default(false)]
	pub check_transfer_allowed: bool,
	#[default(CLONE_ID_INVALID)]
	pub clone_id: CloneId,
	#[default(f32::MAX)]
	pub pos_x: f32,
	#[default(f32::MAX)]
	pub pos_y: f32,
	#[default(f32::MAX)]
	pub pos_z: f32,
	#[default(1.0)]
	pub rot_w: f32,
	#[default(0.0)]
	pub rot_x: f32,
	#[default(0.0)]
	pub rot_y: f32,
	#[default(0.0)]
	pub rot_z: f32,
	pub spawn_point: GmWString,
	pub instance_type: u8, // todo: type
	#[default(MAP_ID_INVALID)]
	pub zone_id: MapId,
}

#[derive(Debug, GameMessage)]
pub struct TransferToZoneCheckedIm {
	#[default(false)]
	pub is_there_a_queue: bool,
	#[default(CLONE_ID_INVALID)]
	pub clone_id: CloneId,
	#[default(f32::MAX)]
	pub pos_x: f32,
	#[default(f32::MAX)]
	pub pos_y: f32,
	#[default(f32::MAX)]
	pub pos_z: f32,
	#[default(1.0)]
	pub rot_w: f32,
	#[default(0.0)]
	pub rot_x: f32,
	#[default(0.0)]
	pub rot_y: f32,
	#[default(0.0)]
	pub rot_z: f32,
	pub spawn_point: GmWString,
	pub uc_instance_type: u8,
	#[default(MAP_ID_INVALID)]
	pub zone_id: MapId,
}

#[derive(Debug, GameMessage)]
pub struct InvalidZoneTransferList {
	pub customer_feedback_url: GmWString,
	pub invalid_map_transfer_list: GmWString,
	pub customer_feedback_on_exit: bool,
	pub customer_feedback_on_invalid_map_transfer: bool,
}

#[derive(Debug, GameMessage)]
pub struct TransferToLastNonInstance {
	#[default(true)]
	pub use_last_position: bool,
	pub player_id: ObjId,
	#[default(f32::MAX)]
	pub pos_x: f32,
	#[default(f32::MAX)]
	pub pos_y: f32,
	#[default(f32::MAX)]
	pub pos_z: f32,
	#[default(1.0)]
	pub rot_w: f32,
	#[default(0.0)]
	pub rot_x: f32,
	#[default(0.0)]
	pub rot_y: f32,
	#[default(0.0)]
	pub rot_z: f32,
}

#[derive(Debug, GameMessage)]
pub struct DisplayMessageBox {
	pub show: bool,
	pub callback_client: ObjId,
	pub identifier: GmWString,
	pub image_id: i32,
	pub text: GmWString,
	pub user_data: GmWString,
}

#[derive(Debug, GameMessage)]
pub struct SetJetPackMode {
	#[default(false)]
	pub bypass_checks: bool,
	#[default(false)]
	pub do_hover: bool,
	pub use_jetpack: bool,
	#[default(-1)]
	pub effect_id: i32,
	#[default(10.0)]
	pub airspeed: f32,
	#[default(15.0)]
	pub max_airspeed: f32,
	#[default(1.0)]
	pub vert_vel: f32,
	#[default(-1)]
	pub warning_effect_id: i32,
}

#[derive(Debug, GameMessage)]
pub struct UseItemResult {
	pub item_template_id: Lot,
	#[default(false)]
	pub use_item_result: bool,
}

#[derive(Debug, GameMessage)]
pub struct SetEmoteLockState {
	pub lock: bool,
	pub emote_id: i32,
}

#[derive(Debug, GameMessage)]
pub struct UseItemRequirementsResponse {
	pub use_response: UseItemResponse,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum UseItemResponse {
	Invalid,
	NoImaginationForPet,
	FailedPrecondition,
	MountsNotAllowed,
}

#[derive(Debug, GameMessage)]
pub struct PlayEmbeddedEffectOnAllClientsNearObject {
	pub effect_name: GmWString,
	pub from_object_id: ObjId,
	pub radius: f32,
}

#[derive(Debug, GameMessage)]
pub struct NotifyClientZoneObject {
	pub name: GmWString,
	pub param1: i32,
	pub param2: i32,
	pub param_obj: ObjId,
	pub param_str: GmString,
}

#[derive(Debug, GameMessage)]
pub struct PropertyRentalResponse {
	pub clone_id: CloneId,
	pub code: i32, // todo: type
	pub property_id: ObjId,
	pub rentdue: i64, // todo: type
}

#[derive(Debug, GameMessage)]
pub struct PlatformResync {
	pub reverse: bool,
	pub stop_at_desired_waypoint: bool,
	pub command: i32,
	pub state: i32, // todo: type
	pub unexpected_command: i32,
	pub idle_time_elapsed: f32,
	pub move_time_elapsed: f32,
	pub percent_between_points: f32,
	pub desired_waypoint_index: i32,
	pub index: i32,
	pub next_index: i32,
	pub unexpected_location: Vector3,
	#[default(Quaternion::IDENTITY)]
	pub unexpected_rotation: Quaternion,
}

#[derive(Debug, GameMessage)]
pub struct PlayCinematic {
	#[default(true)]
	pub allow_ghost_updates: bool,
	pub close_multi_interact: bool,
	pub send_server_notify: bool,
	#[default(false)]
	pub use_controlled_object_for_audio_listener: bool,
	#[default(EndBehavior::Return)]
	pub end_behavior: EndBehavior,
	#[default(false)]
	pub hide_player_during_cine: bool,
	#[default(-1.0)]
	pub lead_in: f32,
	#[default(false)]
	pub leave_player_locked_when_finished: bool,
	#[default(true)]
	pub lock_player: bool,
	pub path_name: GmWString,
	#[default(false)]
	pub result: bool,
	#[default(false)]
	pub skip_if_same_path: bool,
	pub start_time_advance: f32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum EndBehavior {
	Return,
	Wait,
}

#[derive(Debug, GameMessage)]
pub struct ServerTradeInvite {
	#[default(false)]
	pub need_invite_pop_up: bool,
	pub requestor: ObjId,
	pub name: GmWString,
}

#[derive(Debug, GameMessage)]
pub struct ServerTradeInitialReply {
	pub invitee: ObjId,
	pub result_type: ResultType,
	pub name: GmWString,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum ResultType {
	NotFound,
	InviteSent,
	OutOfRange,
	AlreadyTrading,
	GeneralError,
}

#[derive(Debug, GameMessage)]
pub struct ServerTradeFinalReply {
	pub result: bool,
	pub invitee: ObjId,
	pub name: GmWString,
}

#[derive(Debug, GameMessage)]
pub struct ServerTradeAccept {
	#[default(false)]
	pub first: bool,
}

#[derive(Debug, GameMessage)]
pub struct RequestClientBounce {
	pub bounce_target_id: ObjId,
	pub bounce_target_pos_on_server: Vector3,
	pub bounced_obj_lin_vel: Vector3,
	pub request_source_id: ObjId,
	pub all_bounced: bool,
	pub allow_client_override: bool,
}

#[derive(Debug, GameMessage)]
pub struct ObjectActivatedClient {
	pub activator_id: ObjId,
	pub object_activated_id: ObjId,
}

#[derive(Debug, GameMessage)]
pub struct NotifyClientObject {
	pub name: GmWString,
	pub param1: i32,
	pub param2: i32,
	pub param_obj: ObjId,
	pub param_str: GmString,
}

#[derive(Debug, GameMessage)]
pub struct ModifyPlayerZoneStatistic {
	#[default(false)]
	pub set: bool,
	pub stat_name: GmWString,
	#[default(0)]
	pub stat_value: i32,
	#[default(MAP_ID_INVALID)]
	pub zone_id: MapId,
}

#[derive(Debug, GameMessage)]
pub struct StartArrangingWithItem {
	#[default(true)]
	pub first_time: bool,
	#[default(OBJID_EMPTY)]
	pub build_area_id: ObjId,
	pub build_start_pos: Vector3,
	pub source_bag: i32, // todo: type
	pub source_id: ObjId,
	pub source_lot: Lot,
	pub source_type: i32, // todo: type
	pub target_id: ObjId,
	pub target_lot: Lot,
	pub target_pos: Vector3,
	pub target_type: i32, // todo: type
}

#[derive(Debug, GameMessage)]
pub struct FinishArrangingWithItem {
	#[default(OBJID_EMPTY)]
	pub build_area_id: ObjId,
	pub new_source_bag: i32, // todo: type
	pub new_source_id: ObjId,
	pub new_source_lot: Lot,
	pub new_source_type: i32, // todo: type
	pub new_target_id: ObjId,
	pub new_target_lot: Lot,
	pub new_target_type: i32, // todo: type
	pub new_target_pos: Vector3,
	pub old_item_bag: i32, // todo: type
	pub old_item_id: ObjId,
	pub old_item_lot: Lot,
	pub old_item_type: i32, // todo: type
}

#[derive(Debug, GameMessage)]
pub struct SetBuildModeConfirmed {
	pub start: bool,
	#[default(true)]
	pub warn_visitors: bool,
	#[default(false)]
	pub mode_paused: bool,
	#[default(1)]
	pub mode_value: i32, // todo: type
	pub player_id: ObjId,
	#[default(Vector3::ZERO)]
	pub start_pos: Vector3,
}

#[derive(Debug, GameMessage)]
pub struct NotifyClientFailedPrecondition {
	pub failed_reason: GmWString,
	pub precondition_id: i32,
}

#[derive(Debug, GameMessage)]
pub struct EchoSyncSkill {
	#[default(false)]
	pub done: bool,
	pub bitstream: Vec<u8>,
	pub behavior_handle: u32,
	pub skill_handle: u32,
}

#[derive(Debug, GameMessage)]
pub struct DoClientProjectileImpact {
	#[default(OBJID_EMPTY)]
	pub org_id: ObjId,
	#[default(OBJID_EMPTY)]
	pub owner_id: ObjId,
	#[default(OBJID_EMPTY)]
	pub target_id: ObjId,
	pub bitstream: Vec<u8>,
}

#[derive(Debug, GameMessage)]
pub struct SetPlayerAllowedRespawn {
	pub dont_prompt_for_respawn: bool,
}

#[derive(Debug, GameMessage)]
pub struct UncastSkill {
	pub skill_id: i32, // todo: type
}

#[derive(Debug, GameMessage)]
pub struct FireEventClientSide {
	pub args: GmWString,
	pub object: ObjId,
	#[default(0)]
	pub param1: i64,
	#[default(-1)]
	pub param2: i32,
	pub sender_id: ObjId,
}

#[derive(Debug, GameMessage)]
pub struct ChangeObjectWorldState {
	#[default(ObjectWorldState::InWorld)]
	pub new_state: ObjectWorldState,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum ObjectWorldState {
	InWorld,
	Attached,
	Inventory,
}

#[derive(Debug, GameMessage)]
pub struct LockNodeRotation {
	pub node_name: GmString,
}

#[derive(Debug, GameMessage)]
pub struct PlayerReachedRespawnCheckpoint {
	pub pos: Vector3,
	#[default(Quaternion::IDENTITY)]
	pub rot: Quaternion,
}

#[derive(Debug, GameMessage)]
pub struct MatchResponse {
	pub response: MatchResponseType,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum MatchResponseType {
	Ok,
	AlreadyJoined,
	NotJoined,
	AlreadyReady,
	AlreadyNotReady,
	Fail,
	TeamFail,
}

#[derive(Debug, GameMessage)]
pub struct SetStatusImmunity {
	pub state_change_type: ImmunityState,
	pub immune_to_basic_attack: bool,
	pub immune_to_dot: bool,
	pub immune_to_imagination_gain: bool,
	pub immune_to_imagination_loss: bool,
	pub immune_to_interrupt: bool,
	pub immune_to_knockback: bool,
	pub immune_to_pull_to_point: bool,
	pub immune_to_quickbuild_interrupt: bool,
	pub immune_to_speed: bool,
}

#[derive(Debug, GameMessage)]
pub struct ModifyLegoScore {
	pub score: i64,
	#[default(LootType::None)]
	pub source_type: LootType,
}

#[derive(Debug, GameMessage)]
pub struct SetRailMovement {
	pub path_go_forward: bool,
	pub path_name: GmWString,
	pub path_start: u32,
	#[default(-1)]
	pub rail_activator_component_id: i32,
	#[default(OBJID_EMPTY)]
	pub rail_activator_obj_id: ObjId,
}

#[derive(Debug, GameMessage)]
pub struct StartRailMovement {
	#[default(true)]
	pub damage_immune: bool,
	#[default(true)]
	pub no_aggro: bool,
	#[default(false)]
	pub notify_activator: bool,
	#[default(true)]
	pub show_name_billboard: bool,
	#[default(true)]
	pub camera_locked: bool,
	#[default(true)]
	pub collision_enabled: bool,
	pub loop_sound: GmWString,
	#[default(true)]
	pub path_go_forward: bool,
	pub path_name: GmWString,
	#[default(0)]
	pub path_start: u32,
	#[default(-1)]
	pub rail_activator_component_id: i32,
	#[default(OBJID_EMPTY)]
	pub rail_activator_obj_id: ObjId,
	pub start_sound: GmWString,
	pub stop_sound: GmWString,
	#[default(true)]
	pub use_db: bool,
}

#[derive(Debug, GameMessage)]
pub struct NotifyRailActivatorStateChange {
	#[default(true)]
	pub active: bool,
}

#[derive(Debug, GameMessage)]
pub struct UpdatePlayerStatistic {
	pub update_id: i32,
	#[default(1)]
	pub update_value: i64,
}

#[derive(Debug, GameMessage)]
pub struct NotifyNotEnoughInvSpace {
	pub free_slots_needed: u32,
	#[default(InventoryType::Default)]
	pub inventory_type: InventoryType,
}

#[derive(Debug, GameMessage)]
pub struct NotifyPropertyOfEditMode {
	pub editing_active: bool,
}

#[derive(Debug, GameMessage)]
pub struct TeamAddPlayer {
	#[default(false)]
	pub is_free_trial: bool,
	#[default(false)]
	pub local: bool,
	#[default(false)]
	pub no_loot_on_death: bool,
	pub player_id: ObjId,
	pub player_name: GmWString,
	#[default(ZoneId::INVALID)]
	pub zone_id: ZoneId,
}

#[derive(Debug, GameMessage)]
pub struct TeamRemovePlayer {
	pub disband: bool,
	pub is_kicked: bool,
	pub is_leaving: bool,
	#[default(false)]
	pub local: bool,
	pub leader_id: ObjId,
	pub player_id: ObjId,
	pub name: GmWString,
}

#[derive(Debug, GameMessage)]
pub struct UpdatePropertyModelCount {
	#[default(0)]
	pub model_count: u32,
}

#[derive(Debug, GameMessage)]
pub struct StartCelebrationEffect {
	pub animation: GmWString,
	#[default(11164)]
	pub background_object: Lot,
	#[default(12458)]
	pub camera_path_lot: Lot,
	#[default(1.0)]
	pub cele_lead_in: f32,
	#[default(0.8)]
	pub cele_lead_out: f32,
	#[default(-1)]
	pub celebration_id: i32,
	pub duration: f32,
	pub icon_id: u32,
	pub main_text: GmWString,
	pub mixer_program: GmString,
	pub music_cue: GmString,
	pub path_node_name: GmString,
	pub sound_guid: GmString,
	pub sub_text: GmWString,
}

#[derive(Debug, GameMessage)]
pub struct SetLocalTeam {
	#[default(false)]
	pub is_local: bool,
}

#[derive(Debug, GameMessage)]
pub struct PlayerSetCameraCyclingMode {
	#[default(true)]
	pub allow_cycling_while_dead_only: bool,
	#[default(CyclingMode::AllowCycleTeammates)]
	pub cycling_mode: CyclingMode,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum CyclingMode {
	AllowCycleTeammates,
	DisallowCycling,
}

#[derive(Debug, GameMessage)]
pub struct SetMountInventoryId {
	#[default(OBJID_EMPTY)]
	pub inventory_mount_id: ObjId,
}

#[derive(Debug, GameMessage)]
pub struct NotifyLevelRewards {
	pub level: i32,
	#[default(false)]
	pub sending_rewards: bool,
}
