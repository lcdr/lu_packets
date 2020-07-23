GameMessage::PlayCinematic(
	PlayCinematic {
		allow_ghost_updates: true,
		close_multi_interact: false,
		send_server_notify: false,
		use_controlled_object_for_audio_listener: false,
		end_behavior: EndBehavior::Return,
		hide_player_during_cine: false,
		lead_in: -1.0,
		leave_player_locked_when_finished: false,
		lock_player: true,
		path_name: lu!("MissionCam"),
		result: false,
		skip_if_same_path: false,
		start_time_advance: 0.0,
	},
)
