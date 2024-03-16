#[cfg(test)]
mod tests {
    use serde::Serialize;
    use euclid::default::Vector2D;
    use sandcore_core::message_client::MessageClient;
    use sandcore_core::message_server::MessageServer;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }

    #[test]
    fn serialize() {
        let value = serde_json::to_value(MessageServer::Err("Something wrong".to_string())).expect("wrong");
        let value = serde_json::to_value(MessageClient::SetDirection(Vector2D::new(2.0, 3.5))).expect("wrong");
    }
}
