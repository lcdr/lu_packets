LuMessage::Auth(
	AuthMessage::LoginRequest(
		LoginRequest {
			username: lu!("pwjones"),
			password: lu!("password"),
			locale_id: 1033,
			client_os: ClientOs::Windows,
			computer_stats: ComputerStats {
				memory_stats: lu!("393355264 p,429252608 vbytes.29 n-use.8346740 TKb-pmem.5876580 FKb pmem.16691632 TKb pfile.13948488 FKb pfile.2097024 TKbytes vmem. \n1453856 FKb vmem.P 393355264 p,429252608 v."),
				video_card_info: lu!("NVIDIA Quadro FX 580 (HAL-PURE MT HWVP)"),
				processor_info: ProcessorInfo {
					number_of_processors: 8,
					processor_type: 586,
					processor_level: 6,
					processor_revision: 7685,
				},
				os_info: OsInfo {
					os_version_info_size: 276,
					major_version: 6,
					minor_version: 1,
					build_number: 7601,
					platform_id: 2,
				},
			},
		},
	),
)
