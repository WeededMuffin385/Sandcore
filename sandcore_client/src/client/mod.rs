use std::io;
use std::io::Read;
use std::net::{TcpStream, ToSocketAddrs};
use sandcore_core::message::Message;
use crate::world::World;

pub struct Client {
    stream: TcpStream,
    world: World,
}


impl Client {
    pub fn new(addr: impl ToSocketAddrs) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        let world = World::default();
        
        Ok(Self { stream, world })
    }

    pub fn update_stream(&mut self) {
        let mut header = Message::default();
        header.read(&mut self.stream);
    }
}