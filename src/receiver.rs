extern crate mio;

use mio::*;
use mio::tcp::*;
use std::net::SocketAddr;

struct Server;

impl Handler for Server {
    type Timeout = usize;
    type Message = ();
}

fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let mut handler = Server;
   
    let server_socket = TcpSocket::v4().unwrap();
    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
    server_socket.bind(&address);
    server_socket.listen(128);

    event_loop.run(&mut handler);
}
