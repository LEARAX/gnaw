use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct Mpd {
    pub connection: std::net::TcpStream,
    pub version: String,
}

impl Mpd {
    pub fn new(address: std::net::SocketAddr) -> Result<Mpd, &'static str> {
        if let Ok(stream) = std::net::TcpStream::connect(address) {
            let mut reader = BufReader::new(&stream);
            let mut buffer = String::new();
            if let Ok(_) = reader.read_line(&mut buffer) {
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
                return Err("Failed to read version information from MPD");
            }
        } else {
            return Err("Failed to connect to MPD");
        }
    }
}
