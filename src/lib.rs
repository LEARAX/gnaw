use std::io::prelude::*;
use std::io::BufReader;

pub struct Mpd {
    pub connection: std::net::TcpStream,
    pub version: String,
}

impl Mpd {
    pub fn new(address: std::net::SocketAddr) -> Result<Mpd, &'static str> {
        if let Ok(stream) = std::net::TcpStream::connect(address) {
            let mut reader = BufReader::new(&stream);
            let mut buffer = String::new();
            reader.read_line(&mut buffer).expect("failed to read initial response from MPD");
                if &buffer[0..6] == "OK MPD" {
                    buffer.pop();
                    Ok(Mpd {
                        connection: stream,
                        version: buffer[7..].to_string(),
                    })
                } else {
                    return Err("MPD returned an inappropriate response");
                }
        } else {
            return Err("failed to connect to MPD");
        }
    }
}
