PhantomPhysicsConstruction {
	position_rotation_info: Some(
		PositionRotationInfo {
			position: Vector3 {
				x: 0.0,
				y: 1.0,
				z: 2.0,
			},
			rotation: crate::world::Quaternion {
				x: -0.703031,
				y: 0.07643375,
				z: 0.07637292,
				w: 0.7029029,
			},
		},
	),
	active_physics_effect_info: Some(
		ActivePhysicsEffectInfo {
			active_physics_effect: Some(
				PhysicsEffectInfo {
					effect_type: PhysicsEffectType::Push,
					amount: 35.0,
					distance_info: Some(
						DistanceInfo {
							min_distance: 3.0,
							max_distance: 4.0,
						}
					),
					impulse_velocity: Some(
						Vector3 {
							x: 5.0,
							y: 6.0,
							z: 7.0,
						},
					),
				},
			),
		},
	),
}
