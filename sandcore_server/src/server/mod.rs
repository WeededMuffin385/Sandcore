mod client;

use std::net::{TcpListener, ToSocketAddrs};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tokio::runtime::{Handle, Runtime};
use client::Client;
use crate::world::World;

use crate::world::message::Message as WorldMessage;
use crate::world::message::Request as WorldRequest;
use crate::world::message::Response as WorldResponse;

pub struct Server {
    listener: TcpListener,
    world: World,
}


impl Server {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Self {
        let listener = TcpListener::bind(addr).expect("unable to bind listener");
        let world = World::new();


        Self {
            listener,
            world,
        }
    }

    pub fn run(mut self) {
        let Self {
            listener,
            world,
        } = self;

        let world_sender = world.get_sender();

        let world_handle = thread::spawn(move || {
            println!("world started");
            run_world(world);
        });

        let listener_handle =  thread::spawn(move || {
            println!("listener started");
            run_listener(listener, world_sender);
        });

        world_handle.join().unwrap();
        listener_handle.join().unwrap();
    }
}

fn run_listener(listener: TcpListener, mut sender_world: mpsc::Sender<WorldMessage>) {
    let runtime = Runtime::new().expect("unable to create runtime");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            println!("accepted");
            let client = Client::new(stream, sender_world.clone());
            runtime.spawn(client.run());
        }
    }
}

fn run_world(mut world: World) {
    loop {
        //let time = Instant::now();
        world.update();

        //let elapsed = time.elapsed().as_secs_f64();
        //if elapsed < 1.0 / 20.0 { thread::sleep(Duration::from_secs_f64(1.0 / 20.0 - elapsed)); }
        //println!("{}", time.elapsed().as_secs_f64());
    }
}