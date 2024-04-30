mod client;

use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use tokio::runtime::Builder;
use sandcore_world::world::{message as world_message, World};
use crate::server::client::Client;

pub fn run() {
	let world = World::new();
	let sender = world.get_sender();

	let join_world = thread::spawn(||{ run_world(world) });
	let join_listener = thread::spawn(||{ run_listener(sender) });

	join_world.join().unwrap();
	join_listener.join().unwrap();
}

fn run_listener(sender_world: Sender<world_message::Message>) {
	let runtime = Builder::new_current_thread().enable_all().build().unwrap();

	runtime.block_on(async {
		let listener = TcpListener::bind("127.0.0.1:3030").await.unwrap();
		loop {
			let client = Client::new(listener.accept().await.unwrap(), sender_world.clone());
			tokio::spawn(async move { client.run().await });
		}
	});
}

fn run_world(mut world: World) {
	let mut instant_tps = Instant::now();
	let mut tps = 0.0;

	let mut output_tps = || {
		let elapsed = instant_tps.elapsed().as_secs_f64();
		if elapsed >= 1.0 {
			println!("tps: {}", tps / elapsed);
			instant_tps = Instant::now();
			tps = 0.0;
		}
		tps += 1.0;
	};


	let target_tps = 64.0;
	let target_duration = 1.0 / target_tps;

	let mut instant = Instant::now();
	let mut current = instant;

	loop {
		output_tps();
		world.update(&current.elapsed());

		let elapsed = current.elapsed().as_secs_f64();
		if elapsed < target_duration {
			let sleep_duration = Duration::from_secs_f64(target_duration - elapsed);
			thread::sleep(sleep_duration);
		}

		current = instant;
		instant = Instant::now();
	}
}