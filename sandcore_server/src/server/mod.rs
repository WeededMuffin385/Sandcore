mod client;

use std::sync::mpsc::Sender;
use std::thread;
use std::time::Instant;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use sandcore_world::world::{message as world_message, World};
use crate::server::client::Client;

struct Server;

pub fn run() {
	let world = World::new();
	let sender = world.get_sender();

	let join_world = thread::spawn(||{ run_world(world) });
	let join_listener = thread::spawn(||{ run_listener(sender) });

	join_world.join().unwrap();
	join_listener.join().unwrap();
}

fn run_listener(sender_world: Sender<world_message::Message>) {
	let runtime = Runtime::new().unwrap();

	runtime.block_on(async {
		let listener = TcpListener::bind("127.0.0.1:3030").await.unwrap();
		loop {
			let client = Client::new(listener.accept().await.unwrap(), sender_world.clone());
			tokio::spawn(async move { client.run().await });
		}
	});
}

fn run_world(mut world: World) {
	let mut instant = Instant::now();
	let mut current = Instant::now();

	loop {
		world.update(&current.elapsed());
		current = instant;
		instant = Instant::now();
	}
}