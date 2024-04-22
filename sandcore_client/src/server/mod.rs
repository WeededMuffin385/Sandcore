use tokio::net::ToSocketAddrs;
use tokio::io;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Server {
	stream: TcpStream,
}

impl Server {
	pub async fn new(addr: impl ToSocketAddrs) -> io::Result<Self> {
		let stream = TcpStream::connect(addr).await?;

		Ok(Self {
			stream,
		})
	}
}