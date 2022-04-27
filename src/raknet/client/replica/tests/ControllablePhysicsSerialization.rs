ControllablePhysicsSerialization {
	cheat_info: Some(CheatInfo {
		gravity_scale: 8.0,
		run_multiplier: 9.0,
	}),
	unknown_1: Some(Unknown1 {
		loot_pickup_radius: 10.0,
		unknown_2: true,
	}),
	unknown_2: Some(Unknown2 {
		unknown_1: Some(Unknown1 {
			loot_pickup_radius: 11.0,
			unknown_2: true,
	}),
	}),
	frame_stats_teleport_info: Some(FrameStatsTeleportInfo {
		frame_stats: FrameStats {
			position: Vector3 {
				x: 12.0,
				y: 13.0,
				z: 14.0,
			},
			rotation: Quaternion {
				x: 15.0,
				y: 16.0,
				z: 17.0,
				w: 18.0,
			},
			is_on_ground: true,
			is_on_rail: true,
			linear_velocity: Some(Vector3 {
				x: 19.0,
				y: 20.0,
				z: 21.0,
			}),
			angular_velocity: Some(Vector3 {
				x: 22.0,
				y: 23.0,
				z: 24.0,
			}),
			local_space_info: Some(LocalSpaceInfo {
				object_id: 25,
				position: Vector3 {
					x: 26.0,
					y: 27.0,
					z: 28.0,
				},
				linear_velocity: Some(Vector3 {
					x: 29.0,
					y: 30.0,
					z: 31.0,
				}),
			}),
		},
		is_teleporting: true,
	}),
}
