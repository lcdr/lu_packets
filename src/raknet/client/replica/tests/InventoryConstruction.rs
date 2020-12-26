InventoryConstruction {
	equipped_items: Some(vec![
		EquippedItemInfo {
			id: 0,
			lot: 1,
			subkey: Some(2),
			count: Some(3),
			slot: Some(4),
			inventory_type: Some(InventoryType::Default),
			extra_info: Some(lnv! {
				"key": 5,
			}),
			is_bound: true,
		},
	].into()),
	equipped_model_transforms: Some(vec![
		EquippedModelTransform {
			model_id: 6,
			equip_position: Vector3 {
				x: 7.0,
				y: 8.0,
				z: 9.0,
			},
			equip_rotation: Quaternion {
				x: 10.0,
				y: 11.0,
				z: 12.0,
				w: 13.0,
			},
		},
	].into()),
}
