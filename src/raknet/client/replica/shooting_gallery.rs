use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use super::scripted_activity::ActivityUserInfo;
use super::{ComponentConstruction, ComponentProtocol, ComponentSerialization};
use crate::common::{LVec, ObjId};
use crate::world::Vector3;

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct ShootingGalleryInfo {
    pub velocity: f64,
    pub cooldown: f64,
    pub min_distance: f64,
    pub muzzle_position: Vector3,
    pub angle_to_fire: f32,
    pub forward: Vector3,
    pub user: ObjId,
    pub activity_timer: f32,
    pub camera_fov: f32,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct ShootingGalleryConstruction {
    pub activity_user_infos: Option<LVec<u32, ActivityUserInfo>>,
    pub camera_position: Vector3,
    pub camera_look_at_position: Vector3,
    pub shooting_gallery_info: Option<ShootingGalleryInfo>,
}

impl ComponentConstruction for ShootingGalleryConstruction {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct ShootingGallerySerialization {
    pub activity_user_infos: Option<LVec<u32, ActivityUserInfo>>,
    pub shooting_gallery_info: Option<ShootingGalleryInfo>,
}

impl ComponentSerialization for ShootingGallerySerialization {
    fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
        self.serialize(writer)
    }
}

pub struct ShootingGalleryProtocol;

impl ComponentProtocol for ShootingGalleryProtocol {
    type Construction = ShootingGalleryConstruction;
    type Serialization = ShootingGallerySerialization;
}
