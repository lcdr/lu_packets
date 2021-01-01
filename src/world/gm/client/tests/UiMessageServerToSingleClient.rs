GameMessage::UiMessageServerToSingleClient(
	UiMessageServerToSingleClient {
		args: amf3! {
			"false": false,
			"true": true,
			"double": 3.14,
			"string": "string",
			"array": amf3! ["inner", "array", true],
		},
		message_name: lu!(b"QueueChoiceBox"),
	},
)
