use std::net::SocketAddr;
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{get, post};

pub fn routes() -> Router {
	Router::new()
	 .route("/registration", post(registration))
	 .route("/authorization", post(authorization))
	 .route("/ws", get(websocket))
}

async fn registration() {

}

async fn authorization() {

}

async fn websocket(
	websocket: WebSocketUpgrade,
	ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
	websocket.on_upgrade(move |socket| handle_websocket(socket, addr))
}

async fn handle_websocket(mut socket: WebSocket, addr: SocketAddr) {
	if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
		println!("Pinged {addr} successfully");
	} else {
		println!("Pinged {addr} falsely");
	}
}