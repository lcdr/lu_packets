MovingPlatformConstruction {
	path_info: Some(PlatformPathInfo {
		path_name: lu!("path"),
		starting_waypoint: 1,
		is_in_reverse: true,
	}),
	subcomponent_infos: Some(vec![
		PlatformSubcomponentInfo::Mover(
			Some(
				PlatformMoverInfo {
					state: 9,
					desired_waypoint_index: 2,
					stop_at_desired_waypoint: true,
					is_in_reverse: true,
					percent_to_next_waypoint: 34.56,
					position: Vector3 {
						x: 7.0,
						y: 8.0,
						z: 9.0,
					},
					current_waypoint_index: 10,
					next_waypoint_index: 11,
					idle_time_elapsed: 12.0,
					move_time_elapsed: 13.0,
				}
			)
		),
	].into()),
}
