use std::net::{SocketAddr, TcpStream};

#[derive(Debug)]
pub struct Mpd {
    connection: std::net::TcpStream,
}

pub fn mpd_connect(address: SocketAddr) -> Mpd {
    if let Ok(stream) = TcpStream::connect(address) {
        Mpd {
            connection: stream
        }
    } else {
        panic!("Failed to connect to MPD!")
    }
}
