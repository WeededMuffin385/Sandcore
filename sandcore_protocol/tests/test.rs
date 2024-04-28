#[cfg(test)]
mod tests {
	use serde::Serialize;
	use euclid::default::Vector2D;
	use sandcore_protocol::message_client::MessageClient;
	use sandcore_protocol::message_server::MessageServer;

	#[test]
	fn it_works() {
		let result = 4;
		assert_eq!(result, 4);
	}

	#[test]
	fn serialize() {
	}
}