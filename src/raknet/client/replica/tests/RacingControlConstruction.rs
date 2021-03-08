RacingControlConstruction {
	activity_user_infos: Some(vec![
		ActivityUserInfo {
			user_object_id: 1152921507960010807,
			activity_value_0: 0.0,
			activity_value_1: 1.0,
			activity_value_2: 2.0,
			activity_value_3: 3.0,
			activity_value_4: 4.0,
			activity_value_5: 5.0,
			activity_value_6: 6.0,
			activity_value_7: 7.0,
			activity_value_8: 8.0,
			activity_value_9: 9.0,
		}
	].into()),
	expected_player_count: Some(2),
	pre_race_player_infos: Some(vec![
		PreRacePlayerInfo {
			player_id: 1152921507960010807,
			vehicle_id: 1152921507960010808,
			starting_position: 0,
			is_ready: false,
		},
		PreRacePlayerInfo {
			player_id: 1152921507960010809,
			vehicle_id: 1152921507960010810,
			starting_position: 1,
			is_ready: true,
		},
	]),
	post_race_player_infos: Some(vec![
		PostRacePlayerInfo {
			player_id: 1152921507960010807,
			current_rank: 0,
		},
		PostRacePlayerInfo {
			player_id: 1152921507960010809,
			current_rank: 1,
		},
	]),
	race_info: Some(RaceInfo {
		lap_count: 3,
		path_name: lu!("race path"),
	}),
	during_race_player_infos: Some(vec![
		DuringRacePlayerInfo {
			player_id: 1152921507960010807,
			best_lap_time: 12.3,
			race_time: 45.6,
		},
		DuringRacePlayerInfo {
			player_id: 1152921507960010809,
			best_lap_time: 56.7,
			race_time: 78.9,
		},
	]),
}
