use std::net::{TcpStream, ToSocketAddrs};
use crate::world::World;

pub struct Client {
    socket: TcpStream,
    world: World,
}


impl Client {
    pub fn new(addr: impl ToSocketAddrs) -> Self {
        let socket = TcpStream::connect(addr).expect("Unable to connect");
        let world = World::new();
        
        Self {
            socket,
            world,
        }
    }
}