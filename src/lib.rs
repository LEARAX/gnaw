use chrono::{offset, Date, DateTime, Duration};
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub struct Mpd {
    pub connection: std::net::TcpStream,
    pub version: String,
}

type Song<'a> = HashMap<&'a str, &'a str>;

impl Mpd {
    pub fn new(address: std::net::SocketAddr) -> Result<Mpd, &'static str> {
        if let Ok(stream) = std::net::TcpStream::connect(address) {
            let mut reader = BufReader::new(&stream);
            let mut buffer = String::new();
            reader
                .read_line(&mut buffer)
                .expect("failed to read initial response from MPD");
            if &buffer[0..6] == "OK MPD" {
                Ok(Mpd {
                    connection: stream,
                    version: buffer[7..buffer.len() - 1].to_string(),
                })
            } else {
                Err("MPD returned an inappropriate response")
            }
        } else {
            Err("failed to connect to MPD")
        }
    }
}
