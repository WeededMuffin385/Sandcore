use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use serde::{Deserialize, Serialize};

type HeaderType = u32;
const HEADER_SIZE: usize = core::mem::size_of::<HeaderType>();

/// TODO: Change blocking sockets for non-blocking, so at least server won't be blocked while reading
pub trait Message: Sized + Serialize + for<'a>Deserialize<'a> {
    fn read(stream: &mut TcpStream) -> Result<Self, io::Error> {
        Ok(serde_json::from_slice(&get_body(stream)?).expect("unable to decode message"))
    }
    fn write(&self, stream: &mut TcpStream) -> io::Result<()> {
        let body = serde_json::to_vec(self).expect("unable to serialize message");
        let length = (body.len() as u32).to_be_bytes();

        stream.write(&length)?;
        stream.write(&body)?;
        Ok(())
    }
}

pub fn get_length(stream: &mut TcpStream) -> io::Result<usize> {
    let mut header_bytes = [0; HEADER_SIZE];
    stream.read_exact(&mut header_bytes)?;
    Ok(HeaderType::from_be_bytes(header_bytes) as usize)
}

pub fn get_body(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    let mut body = vec![0; get_length(stream)?];
    stream.read_exact(&mut body)?;
    Ok(body)
}
