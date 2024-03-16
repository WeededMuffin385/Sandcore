use crate::server::Server;

pub mod world;
mod server;

fn main() {
    Server::new("127.0.0.1:4000").run();
    println!("Hello, world!");
}
