extern crate tokio;
extern crate bytes;
#[macro_use]
extern crate futures;

use tokio::io::AsyncWrite;
use tokio::net::{TcpStream, tcp::ConnectFuture};
use bytes::{Bytes, Buf};
use futures::{Future, Async, Poll};
use std::io::{self, Cursor};

// struct GetPeerAddr {
//     connect: ConnectFuture,
// }
// 
// impl Future for GetPeerAddr {
//     type Item = ();
//     type Error = ();
// 
//     fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
//         match self.connect.poll() {
//             Ok(Async::Ready(socket)) => {
//                 println!("peer address = {}", socket.peer_addr().unwrap());
//                 Ok(Async::Ready(()))
//             }
//             Ok(Async::NotReady) => Ok(Async::NotReady),
//             Err(e) => {
//                 println!("failed to connect: {}", e);
//                 Ok(Async::Ready(()))
//             }
//         }
//     }
// }
// 
// fn main() {
//     let addr = "192.168.0.1:1234".parse().unwrap();
//     let connect_future = TcpStream::connect(&addr);
//     let get_peer_addr = GetPeerAddr {
//         connect: connect_future,
//     };
//     tokio::run(get_peer_addr);
// }
//

enum HelloWorld {
    Connecting(ConnectFuture),
    Connected(TcpStream, Cursor<Bytes>),
}

impl Future for HelloWorld {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        use self::HelloWorld::*;

        loop {
            match self {
                Connecting(ref mut f) => {
                    let socket = try_ready!(f.poll());
                    let data = Cursor::new(Bytes::from_static(b"hello world"));
                    *self = Connected(socket, data);
                }
                Connected(ref mut socket, ref mut data) => {
                    while data.has_remaining() {
                        try_ready!(socket.write_buf(data));
                    }
                    return Ok(Async::Ready(()));
                }
            }
        }
    }
}

fn main() {
    let addr = "127.0.0.1:1234".parse().unwrap();
    let connect_future = TcpStream::connect(&addr);
    let hello_world = HelloWorld::Connecting(connect_future);

    tokio::run(hello_world.map_err(|e| println!("{0}", e)))
}
