SimplePhysicsConstruction {
	climbing_property: Some(ClimbingProperty::ClimbWallStick),
	velocity_info: Some(
		VelocityInfo {
			linear_velocity: Vector3 {
				x: 0.0,
				y: 1.0,
				z: 2.0,
			},
			angular_velocity: Vector3 {
				x: 3.0,
				y: 4.0,
				z: 5.0,
			},
		},
	),
	motion_type: Some(
		MotionType::Fixed,
	),
	position_rotation_info: Some(
		PositionRotationInfo {
			position: Vector3 {
				x: 6.0,
				y: 7.0,
				z: 8.0,
			},
			rotation: Quaternion {
				x: 0.0,
				y: 0.53361946,
				z: 0.0,
				w: 0.8457247,
			},
		},
	),
}
