mod client;

use std::net::{TcpListener, ToSocketAddrs};
use tokio::runtime::Runtime;
use client::Client;
use crate::world::World;

pub struct Server {
    listener: TcpListener,
    runtime: Runtime,
    world: World,
}


impl Server {
    fn listen(&mut self) {
        for stream in self.listener.incoming() {
            if let Ok(stream) = stream {
                let sender = self.world.creatures.spawn();
                let client = Client::new(stream, sender);
                client.run(self.runtime.handle());
            } else {
                println!("Connection failed");
            }
        }
    }

    pub fn new<A: ToSocketAddrs>(addr: A) -> Self {
        let listener = TcpListener::bind(addr).expect("unable to bind listener");
        let runtime = Runtime::new().expect("unable to create runtime");
        let world = Default::default();

        Self {
            listener,
            runtime,
            world,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.listen();
        }
    }
}