VehiclePhysicsConstruction {
	vehicle_frame_stats: Some(VehicleFrameStats {
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
		remote_input_info: Some(RemoteInputInfo {
			remote_input_x: 32.0,
			remote_input_y: 33.0,
			do_powerslide: true,
			is_modified: true,
		}),
		remote_input_ping: 34.0,
	}),
	end_of_race_behavior_type: EndOfRaceBehaviorType::Do360Left,
	is_input_locked: true,
	wheel_lock_extra_friction: Some(true),
}
