use std::cmp::PartialEq;

use endio::{Deserialize, Serialize};
use lu_packets_derive::{GameMessage, GmParam, VariantTests};

use crate::common::{ObjId, OBJID_EMPTY};

use super::super::{Lot, LOT_NULL, Quaternion, Vector3};
use super::{EquipInventory, GmString, GmWString, InventoryType, KillType, UnEquipInventory, MissionState, PetNotificationType, MoveItemInInventory, MoveInventoryBatch, SetIgnoreProjectileCollision};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SubjectGameMessage {
	pub subject_id: ObjId,
	pub message: GameMessage,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, VariantTests)]
#[repr(u16)]
pub enum GameMessage {
	RequestDie(RequestDie) = 38,
	PlayEmote(PlayEmote) = 41,
	StartSkill(StartSkill) = 119,
	CasterDead(CasterDead) = 120,
	VerifyAck(VerifyAck) = 121,
	SelectSkill(SelectSkill) = 124,
	PickupCurrency(PickupCurrency) = 137,
	PickupItem(PickupItem) = 139,
	RequestResurrect = 159,
	PopEquippedItemsState = 192,
	RebuildCancel(RebuildCancel) = 209,
	MoveItemInInventory(MoveItemInInventory) = 224,
	EquipInventory(EquipInventory) = 231,
	UnEquipInventory(UnEquipInventory) = 233,
	RespondToMission(RespondToMission) = 249,
	ServerTerminateInteraction(ServerTerminateInteraction) = 358,
	RequestUse(RequestUse) = 364,
	BuyFromVendor(BuyFromVendor) = 373,
	SellToVendor(SellToVendor) = 374,
	CancelDonationOnPlayer = 379,
	AcknowledgePossession(AcknowledgePossession) = 391,
	RequestActivityExit(RequestActivityExit) = 404,
	ShootingGalleryFire(ShootingGalleryFire) = 411,
	RequestVendorStatusUpdate = 416,
	ClientItemConsumed(ClientItemConsumed) = 428,
	UpdateShootingGalleryRotation(UpdateShootingGalleryRotation) = 448,
	SetTooltipFlag(SetTooltipFlag) = 469,
	SetFlag(SetFlag) = 471,
	HasBeenCollected(HasBeenCollected) = 486,
	DespawnPet(DespawnPet) = 499,
	PlayerLoaded(PlayerLoaded) = 505,
	RequestLinkedMission(RequestLinkedMission) = 515,
	MissionDialogueOk(MissionDialogueOk) = 520,
	MessageBoxRespond(MessageBoxRespond) = 530,
	ChoiceBoxRespond(ChoiceBoxRespond) = 531,
	UseNonEquipmentItem(UseNonEquipmentItem) = 603,
	FetchModelMetadataRequest(FetchModelMetadataRequest) = 638,
	CommandPet(CommandPet) = 640,
	RequestActivitySummaryLeaderboardData(RequestActivitySummaryLeaderboardData) = 648,
	NotifyPet(NotifyPet) = 660,
	StartServerPetMinigameTimer = 662,
	ClientExitTamingMinigame(ClientExitTamingMinigame) = 663,
	PetTamingMinigameResult(PetTamingMinigameResult) = 667,
	NotifyTamingBuildSuccess(NotifyTamingBuildSuccess) = 673,
	RequestSetPetName(RequestSetPetName) = 683,
	CinematicUpdate(CinematicUpdate) = 764,
	FireEventServerSide(FireEventServerSide) = 770,
	QueryPropertyData = 717,
	PropertyEditorBegin(PropertyEditorBegin) = 724,
	PropertyEditorEnd = 725,
	RequestPlatformResync = 760,
	ToggleGhostReferenceOverride(ToggleGhostReferenceOverride) = 767,
	SetGhostReferencePosition(SetGhostReferencePosition) = 768,
	UpdateModelFromClient(UpdateModelFromClient) = 793,
	DeleteModelFromClient(DeleteModelFromClient) = 794,
	EnterProperty1(EnterProperty1) = 840,
	PropertyEntranceSync(PropertyEntranceSync) = 842,
	ParseChatMessage(ParseChatMessage) = 850,
	SetMissionTypeState(SetMissionTypeState) = 851,
	ClientTradeRequest(ClientTradeRequest) = 868,
	ClientTradeCancel = 878,
	ClientTradeAccept(ClientTradeAccept) = 880,
	ReadyForUpdates(ReadyForUpdates) = 888,
	SetLastCustomBuild(SetLastCustomBuild) = 890,
	SetIgnoreProjectileCollision(SetIgnoreProjectileCollision) = 903,
	PropertyModerationAction(PropertyModerationAction) = 915,
	BounceNotification(BounceNotification) = 932,
	MoveInventoryBatch(MoveInventoryBatch) = 957,
	SetBbbAutosave(SetBbbAutosave) = 996,
	BbbLoadItemRequest(BbbLoadItemRequest) = 1000,
	BbbSaveRequest(BbbSaveRequest) = 1001,
	BbbResetMetadataSourceItem = 1004,
	ZoneSummaryDismissed(ZoneSummaryDismissed) = 1044,
	ActivityStateChangeRequest(ActivityStateChangeRequest) = 1053,
	StartBuildingWithItem(StartBuildingWithItem) = 1057,
	DoneArrangingWithItem(DoneArrangingWithItem) = 1063,
	SetBuildMode(SetBuildMode) = 1068,
	BuildModeSet(BuildModeSet) = 1069,
	BuildExitConfirmation(BuildExitConfirmation) = 1072,
	MoveItemBetweenInventoryTypes(MoveItemBetweenInventoryTypes) = 1093,
	MissionDialogueCancelled(MissionDialogueCancelled) = 1129,
	ModuleAssemblyQueryData = 1132,
	SyncSkill(SyncSkill) = 1145,
	RequestServerProjectileImpact(RequestServerProjectileImpact) = 1148,
	ToggleSendingPositionUpdates(ToggleSendingPositionUpdates) = 1166,
	PlacePropertyModel(PlacePropertyModel) = 1170,
	ReportBug(ReportBug) = 1198,
	RequestSmashPlayer = 1202,
	ResyncEquipment = 1238,
	RacingPlayerInfoResetFinished(RacingPlayerInfoResetFinished) = 1255,
	VehicleSetWheelLockState(VehicleSetWheelLockState) = 1273,
	PropertyContentsFromClient(PropertyContentsFromClient) = 1305,
	VehicleNotifyServerAddPassiveBoostAction = 1342,
	VehicleNotifyServerRemovePassiveBoostAction = 1343,
	ZonePropertyModelRotated(ZonePropertyModelRotated) = 1370,
	ZonePropertyModelRemovedWhileEquipped(ZonePropertyModelRemovedWhileEquipped) = 1371,
	ZonePropertyModelEquipped(ZonePropertyModelEquipped) = 1372,
	RacingClientReady(RacingClientReady) = 1393,
	ResetPropertyBehaviors(ResetPropertyBehaviors) = 1406,
	SetConsumableItem(SetConsumableItem) = 1409,
	UsedInformationPlaque(UsedInformationPlaque) = 1419,
	ActivateBrickMode(ActivateBrickMode) = 1438,
	CancelRailMovement(CancelRailMovement) = 1474,
	ClientRailMovementReady = 1476,
	PlayerRailArrivedNotification(PlayerRailArrivedNotification) = 1477,
	RequestRailActivatorState = 1479,
	ModifyGhostingDistance(ModifyGhostingDistance) = 1485,
	ModularAssemblyNifCompleted(ModularAssemblyNifCompleted) = 1498,
	GetHotPropertyData = 1511,
	UpdatePropertyPerformanceCost(UpdatePropertyPerformanceCost) = 1547,
	SetEmotesEnabled(SetEmotesEnabled) = 1577,
	VehicleNotifyHitImaginationServer(VehicleNotifyHitImaginationServer) = 1606,
	CelebrationCompleted = 1632,
	NotifyServerLevelProcessingComplete = 1734,
	ServerCancelMoveSkill = 1746,
	DismountComplete(DismountComplete) = 1756,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct RequestDie {
	pub unknown: bool,
	pub death_type: GmWString,
	pub direction_relative_angle_xz: f32,
	pub direction_relative_angle_y: f32,
	pub direction_relative_force: f32,
	#[default(KillType::Violent)]
	pub kill_type: KillType,
	pub killer_id: ObjId,
	pub loot_owner_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct PlayEmote {
	pub emote_id: i32,
	pub target_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct StartSkill {
	#[default(false)]
	pub used_mouse: bool,
	#[default(OBJID_EMPTY)]
	pub consumable_item_id: ObjId,
	#[default(0.0)]
	pub caster_latency: f32,
	#[default(0)]
	pub cast_type: i32, // todo: type
	#[default(Vector3::ZERO)]
	pub last_clicked_posit: Vector3,
	pub optional_originator_id: ObjId,
	#[default(OBJID_EMPTY)]
	pub optional_target_id: ObjId,
	#[default(Quaternion::IDENTITY)]
	pub originator_rot: Quaternion,
	pub bitstream: Vec<u8>,
	pub skill_id: u32, // todo: type
	#[default(0)]
	pub skill_handle: u32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct CasterDead {
	#[default(OBJID_EMPTY)]
	pub caster: ObjId,
	#[default(0)]
	pub skill_handle: u32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct VerifyAck {
	#[default(false)]
	pub different: bool,
	pub bitstream: Vec<u8>,
	#[default(0)]
	pub handle: u32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct SelectSkill {
	#[default(false)]
	pub from_skill_set: bool,
	pub skill_id: i32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct PickupCurrency {
	pub currency: u32,
	pub position: Vector3,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct PickupItem {
	pub loot_object_id: ObjId,
	pub player_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct RebuildCancel {
	pub early_release: bool,
	pub user_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct RespondToMission {
	pub mission_id: i32,
	pub player_id: ObjId,
	pub receiver: ObjId,
	#[default(LOT_NULL)]
	pub reward_item: Lot,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ServerTerminateInteraction {
	pub obj_id_terminator: ObjId,
	pub terminate_type: TerminateType,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum TerminateType {
	Range,
	User,
	FromInteraction,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct RequestUse {
	pub is_multi_interact_use: bool,
	pub multi_interact_id: u32,
	pub multi_interact_type: InteractionType,
	pub object: ObjId,
	#[default(false)]
	pub secondary: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum InteractionType {
	MissionOfferer,
	Vendor,
	ItemRecipe,
	ItemForge,
	Script = 999,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct BuyFromVendor {
	#[default(false)]
	pub confirmed: bool,
	#[default(1)]
	pub count: i32, // todo: unsigned?
	pub item: Lot,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct SellToVendor {
	#[default(1)]
	pub count: i32, // todo: unsigned?
	pub item_obj_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct AcknowledgePossession {
	#[default(OBJID_EMPTY)]
	pub possessed_obj_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct RequestActivityExit {
	pub user_cancel: bool,
	pub user_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ShootingGalleryFire {
	pub target_pos: Vector3,
	pub w: f32,
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ClientItemConsumed {
	pub item: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct UpdateShootingGalleryRotation {
	pub angle: f32,
	pub facing: Vector3,
	pub muzzle_pos: Vector3,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct SetTooltipFlag {
	pub flag: bool,
	pub tool_tip: i32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct SetFlag {
	pub flag: bool,
	pub flag_id: i32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct HasBeenCollected {
	pub player_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct DespawnPet {
	pub delete_pet: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct PlayerLoaded {
	pub player_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct RequestLinkedMission {
	pub player_id: ObjId,
	pub mission_id: i32,
	pub mission_offered: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct MissionDialogueOk {
	pub is_complete: bool,
	pub mission_state: MissionState,
	pub mission_id: i32,
	pub responder: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct MessageBoxRespond {
	pub button: i32,
	pub identifier: GmWString,
	pub user_data: GmWString,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ChoiceBoxRespond {
	pub button_identifier: GmWString,
	pub button: i32,
	pub identifier: GmWString,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct UseNonEquipmentItem {
	pub item_to_use: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct FetchModelMetadataRequest {
	pub context: i32,
	pub object_id: ObjId,
	pub requestor_id: ObjId,
	pub ug_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct CommandPet {
	pub generic_pos_info: Vector3,
	pub obj_id_source: ObjId,
	pub pet_command_type: i32,
	pub type_id: i32,
	#[default(false)]
	pub override_obey: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct RequestActivitySummaryLeaderboardData {
	#[default(0)]
	pub game_id: i32,
	#[default(QueryType::TopCharacter)]
	pub query_type: QueryType,
	#[default(10)]
	pub results_end: i32,
	#[default(0)]
	pub results_start: i32,
	pub target: ObjId,
	pub weekly: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct NotifyPet {
	pub obj_id_source: ObjId,
	pub obj_to_notify_pet_about: ObjId,
	pub pet_notification_type: PetNotificationType,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum QueryType {
	TopAll,
	TopCharacter,
	TopSocial,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ClientExitTamingMinigame {
	#[default(true)]
	pub voluntary_exit: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct PetTamingMinigameResult {
	pub success: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct NotifyTamingBuildSuccess {
	pub build_position: Vector3,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct RequestSetPetName {
	pub name: GmWString,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct CinematicUpdate {
	#[default(CinematicEvent::Started)]
	pub event: CinematicEvent,
	#[default(-1.0)]
	pub overall_time: f32,
	pub path_name: GmWString,
	#[default(-1.0)]
	pub path_time: f32,
	#[default(-1)]
	pub waypoint: i32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum CinematicEvent {
	Started,
	Waypoint,
	Ended,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct FireEventServerSide {
	pub args: GmWString,
	#[default(-1)]
	pub param1: i32,
	#[default(-1)]
	pub param2: i32,
	#[default(-1)]
	pub param3: i32,
	pub sender_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct PropertyEditorBegin {
	#[default(0)]
	pub distance_type: i32,
	#[default(OBJID_EMPTY)]
	pub property_object_id: ObjId,
	#[default(1)]
	pub start_mode: i32,
	#[default(0)]
	pub start_paused: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ToggleGhostReferenceOverride {
	#[default(false)]
	pub ref_override: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct SetGhostReferencePosition {
	pub pos: Vector3,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct UpdateModelFromClient {
	pub model_id: ObjId,
	pub position: Vector3,
	#[default(Quaternion::IDENTITY)]
	pub rotation: Quaternion,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct DeleteModelFromClient {
	#[default(OBJID_EMPTY)]
	pub model_id: ObjId,
	#[default(DeleteReason::PickingModelUp)]
	pub reason: DeleteReason,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum DeleteReason {
	PickingModelUp,
	ReturningModelToInventory,
	BreakingModelApart,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct EnterProperty1 {
	pub index: i32,
	#[default(true)]
	pub return_to_zone: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct PropertyEntranceSync {
	pub include_null_address: bool,
	pub include_null_description: bool,
	pub players_own: bool,
	pub update_ui: bool,
	pub num_results: i32,
	pub reputation_time: i32,
	pub sort_method: i32,
	pub start_index: i32,
	pub filter_text: GmString,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ParseChatMessage {
	pub client_state: i32,
	pub string: GmWString,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct SetMissionTypeState {
	#[default(MissionLockState::New)]
	pub state: MissionLockState,
	pub mission_subtype: GmString,
	pub mission_type: GmString,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum MissionLockState {
	Locked,
	New,
	Unlocked,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ClientTradeRequest {
	#[default(false)]
	pub need_invite_pop_up: bool,
	pub invitee: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ClientTradeAccept {
	#[default(false)]
	pub first: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ReadyForUpdates {
	pub object_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct SetLastCustomBuild {
	pub tokenized_lot_list: GmWString,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct PropertyModerationAction {
	#[default(0)]
	pub character_id: ObjId,
	pub info: GmWString,
	#[default(-1)]
	pub new_moderation_status: i32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct BounceNotification {
	pub obj_id_bounced: ObjId,
	pub obj_id_bouncer: ObjId,
	pub success: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct SetBbbAutosave {
	pub lxfml_data_compressed: Vec<u8>,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct BbbLoadItemRequest {
	pub item_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct BbbSaveRequest {
	pub local_id: ObjId,
	pub lxfml_data_compressed: Vec<u8>,
	pub time_taken_in_ms: u32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ZoneSummaryDismissed {
	pub player_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ActivityStateChangeRequest {
	pub obj_id: ObjId,
	pub num_value_1: i32,
	pub num_value_2: i32,
	pub string_value: GmWString,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct StartBuildingWithItem {
	#[default(true)]
	pub first_time: bool,
	pub success: bool,
	pub source_bag: InventoryType,
	pub source_id: ObjId,
	pub source_lot: Lot,
	pub source_type: i32,
	pub target_id: ObjId,
	pub target_lot: Lot,
	pub target_pos: Vector3,
	pub target_type: i32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct DoneArrangingWithItem {
	pub new_source_bag: InventoryType,
	pub new_source_id: ObjId,
	pub new_source_lot: Lot,
	pub new_source_type: i32,
	pub new_target_id: ObjId,
	pub new_target_lot: Lot,
	pub new_target_type: i32,
	pub new_target_pos: Vector3,
	pub old_item_bag: InventoryType,
	pub old_item_id: ObjId,
	pub old_item_lot: Lot,
	pub old_item_type: i32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct SetBuildMode {
	pub start: bool,
	#[default(-1)]
	pub distance_type: i32,
	#[default(false)]
	pub mode_paused: bool,
	#[default(1)]
	pub mode_value: i32,
	pub player_id: ObjId,
	#[default(Vector3::ZERO)]
	pub start_pos: Vector3,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct BuildModeSet {
	pub start: bool,
	#[default(-1)]
	pub distance_type: i32,
	#[default(false)]
	pub mode_paused: bool,
	#[default(1)]
	pub mode_value: i32,
	pub player_id: ObjId,
	#[default(Vector3::ZERO)]
	pub start_pos: Vector3,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct BuildExitConfirmation {
	pub player_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct MoveItemBetweenInventoryTypes {
	pub inventory_type_a: InventoryType,
	pub inventory_type_b: InventoryType,
	pub object_id: ObjId,
	#[default(true)]
	pub show_flying_loot: bool,
	#[default(1)]
	pub stack_count: u32,
	#[default(LOT_NULL)]
	pub template_id: Lot,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct MissionDialogueCancelled {
	pub is_complete: bool,
	pub mission_state: MissionState,
	pub mission_id: i32,
	pub responder: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct SyncSkill {
	#[default(false)]
	pub done: bool,
	pub bitstream: Vec<u8>,
	pub behavior_handle: u32,
	pub skill_handle: u32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct RequestServerProjectileImpact {
	#[default(OBJID_EMPTY)]
	pub local_id: ObjId,
	#[default(OBJID_EMPTY)]
	pub target_id: ObjId,
	pub bitstream: Vec<u8>,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ToggleSendingPositionUpdates {
	#[default(false)]
	pub send_updates: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct PlacePropertyModel {
	pub model_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ReportBug {
	pub body: GmWString,
	pub client_version: GmString,
	pub other_player_id: GmString,
	pub selection: GmString,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct RacingPlayerInfoResetFinished {
	pub player_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct VehicleSetWheelLockState {
	#[default(true)]
	pub extra_friction: bool,
	#[default(false)]
	pub locked: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct PropertyContentsFromClient {
	#[default(false)]
	pub query_db: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ZonePropertyModelRotated {
	#[default(OBJID_EMPTY)]
	pub player_id: ObjId,
	#[default(OBJID_EMPTY)]
	pub property_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ZonePropertyModelRemovedWhileEquipped {
	#[default(OBJID_EMPTY)]
	pub player_id: ObjId,
	#[default(OBJID_EMPTY)]
	pub property_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ZonePropertyModelEquipped {
	#[default(OBJID_EMPTY)]
	pub player_id: ObjId,
	#[default(OBJID_EMPTY)]
	pub property_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct RacingClientReady {
	pub player_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ResetPropertyBehaviors {
	#[default(true)]
	pub force: bool,
	#[default(false)]
	pub pause: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct SetConsumableItem {
	pub item_template_id: Lot,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct UsedInformationPlaque {
	pub plaque: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ActivateBrickMode {
	#[default(OBJID_EMPTY)]
	pub build_object_id: ObjId,
	#[default(BuildType::OnProperty)]
	pub build_type: BuildType,
	#[default(true)]
	pub enter_build_from_world: bool,
	#[default(true)]
	pub enter_flag: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, GmParam)]
#[repr(u32)]
pub enum BuildType {
	Nowhere,
	InWorld,
	OnProperty,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct CancelRailMovement {
	#[default(false)]
	pub immediate: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct PlayerRailArrivedNotification {
	pub path_name: GmWString,
	pub waypoint_number: i32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ModifyGhostingDistance {
	#[default(1.0)]
	pub distance: f32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct ModularAssemblyNifCompleted {
	pub object_id: ObjId,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct UpdatePropertyPerformanceCost {
	#[default(0.0)]
	pub performance_cost: f32,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct SetEmotesEnabled {
	#[default(true)]
	pub enable_emotes: bool,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct VehicleNotifyHitImaginationServer {
	#[default(OBJID_EMPTY)]
	pub pickup_obj_id: ObjId,
	#[default(OBJID_EMPTY)]
	pub pickup_spawner_id: ObjId,
	#[default(-1)]
	pub pickup_spawner_index: i32,
	#[default(Vector3::ZERO)]
	pub vehicle_position: Vector3,
}

#[derive(Debug, GameMessage, PartialEq)]
pub struct DismountComplete {
	pub mount_id: ObjId,
}
