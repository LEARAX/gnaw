use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct Mpd {
    pub connection: std::net::TcpStream,
    pub version: String,
}

type Song<'a> = HashMap<String, String>;

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
    pub fn current_song(&mut self) -> Result<Song, &'static str> {
        self.connection
            .write(b"currentsong\n")
            .expect("failed to write to MPD connection");

        let reader = BufReader::new(&self.connection);
        let mut current_song = Song::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            if line == "OK" {
                break;
            }
            let split: std::vec::Vec<&str> = line.split(":").collect();
            current_song.insert(split[0].to_string(), split[1][1..].to_string());
        }
        Ok(current_song)
    }
}
