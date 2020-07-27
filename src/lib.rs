use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::net::{SocketAddr, TcpStream};

#[derive(Debug)]
pub struct Mpd {
    pub connection: std::net::TcpStream,
    pub version: String,
}

type Song<'a> = HashMap<String, String>;

impl Mpd {
    pub fn new(address: SocketAddr) -> Result<Mpd, &'static str> {
        if let Ok(stream) = TcpStream::connect(address) {
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
                // TODO Handle failure response
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
            // TODO Handle failure response
            if line == "OK" {
                break;
            }
            let split: std::vec::Vec<&str> = line.split(":").collect();
            current_song.insert(split[0].to_string(), split[1][1..].to_string());
        }
        if current_song.is_empty() {
            Err("no current song")
        } else {
            Ok(current_song)
        }
    }
    pub fn status(&mut self) -> Result<HashMap<String, String>, &'static str> {
        self.connection
            .write(b"status\n")
            .expect("failed to write to MPD connection");

        let reader = BufReader::new(&self.connection);
        let mut status = HashMap::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            // TODO Handle failure response
            if line == "OK" {
                break;
            }
            let split: std::vec::Vec<&str> = line.split(":").collect();
            status.insert(split[0].to_string(), split[1][1..].to_string());
        }
        Ok(status)
    }
    pub fn stats(&mut self) -> Result<HashMap<String, String>, &'static str> {
        self.connection
            .write(b"stats\n")
            .expect("failed to write to MPD connection");

        let reader = BufReader::new(&self.connection);
        let mut stats = HashMap::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            if line == "OK" {
                break;
            }
            let split: std::vec::Vec<&str> = line.split(":").collect();
            stats.insert(split[0].to_string(), split[1][1..].to_string());
        }
        Ok(stats)
    }
    pub fn clear_error(&mut self) {
        self.connection
            .write(b"clearerror\n")
            .expect("failed to write to MPD connection");

        let mut response = String::new();
        self.connection
            .read_to_string(&mut response)
            .expect("failed to read from MPD connection");

        // TODO Handle failure response
        if response == "OK" {
            return;
        } else {
            panic!("MPD returned invalid response on clearerror");
        }
    }
}
