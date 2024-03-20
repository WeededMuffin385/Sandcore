use std::io::Read;
use std::net::TcpStream;

const HEADER_SIZE: usize = core::mem::size_of::<u32>();

#[derive(Default)]
pub struct Message {
    header: [u8; HEADER_SIZE],
    body: Vec<u8>,
}
impl Message {
    fn get_length(&self) -> usize {
        u32::from_be_bytes(self.header) as usize
    }

    fn read_header(&mut self, stream: &mut TcpStream) {
        stream.read_exact(&mut self.header).expect("unable to read header");
    }

    fn read_body(&mut self, stream: &mut TcpStream) {
        self.body = vec![0; self.get_length()];
        stream.read_exact(&mut self.body).expect("unable to read body");
    }

    pub fn read(&mut self, stream: &mut TcpStream) {
        self.read_header(stream);
        self.read_body(stream);
    }

    pub fn get(&self) -> &Vec<u8> {
        &self.body
    }
}
