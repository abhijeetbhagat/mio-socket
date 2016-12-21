extern crate mio;
use mio::{PollOpt, Token, Ready};
use mio::deprecated::{EventLoop, Handler, };
use mio::udp::UdpSocket;
use std::net::{Ipv4Addr, IpAddr, SocketAddr};

struct UdpConnection{
    socket : UdpSocket,
    token : Token,
    interest : Ready

}
struct UdpEndServer{
    socket : UdpSocket,
    token : Token,
    client : Option<UdpConnection> 
}
impl UdpEndServer{
    fn new()->Self{
        UdpEndServer{
            socket : UdpSocket::bind(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)).unwrap(),
            token : Token(0),
            client : None 
        }
    }
}
impl Handler for UdpEndServer{
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop : &mut EventLoop<UdpEndServer>, token : Token, event_set : Ready){
        if event_set.is_readable(){
            if self.token == token{
                let mut buf = [0u8; 512];
                let (amt, src) = self.socket.recv_from(&mut buf).unwrap().unwrap();
                //src.send_to()


            }
        }

    }
}

struct UdpEndClient{

}
fn main() {
    let mut event_loop = EventLoop::<UdpEndServer>::new().unwrap();
    let mut server = UdpEndServer::new();
    event_loop.register(&server.socket, server.token, Ready::readable(), PollOpt::edge());
}
