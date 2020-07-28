use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    error::Error,
    fmt,
    io::{prelude::*, BufReader},
    net::{SocketAddr, TcpStream},
};

#[derive(Debug)]
pub struct Mpd {
    pub connection: std::net::TcpStream,
    pub version: String,
}

// TODO Implement error trait
#[derive(Debug)]
pub struct MpdError {
    code: usize,
    list_position: usize,
    current_command: String,
    message_text: String,
}

type Song<'a> = HashMap<String, String>;

impl fmt::Display for MpdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "code: {}, position: {}, command: {}, message: {}",
            self.code, self.list_position, self.current_command, self.message_text
        )
    }
}

impl Mpd {
    fn handle_error(response: &str) -> MpdError {
        lazy_static! {
            static ref MPD_ERROR_REGEX: Regex =
                Regex::new(r"ACK \[(\d+)@(\d+)\] \{(.+)\} (.+)").unwrap();
        }

        let caps = MPD_ERROR_REGEX.captures(response).unwrap();
        MpdError {
            // I can't tell if this is beautiful or ugly
            // It's probably not good
            code: caps.get(1).unwrap().as_str().parse().unwrap(),
            list_position: caps.get(2).unwrap().as_str().parse().unwrap(),
            current_command: caps.get(3).unwrap().as_str().to_string(),
            message_text: caps.get(4).unwrap().as_str().to_string(),
        }
    }
    // TODO Implement MpdError
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
    // TODO Implement MpdError
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
    // TODO Implement MpdError
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
    pub fn stats(&mut self) -> Result<HashMap<String, String>, MpdError> {
        self.connection
            .write(b"stats\n")
            .expect("failed to write to MPD connection");

        let reader = BufReader::new(&self.connection);
        let mut stats = HashMap::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            if line == "OK" {
                break;
            } else if &line[0..3] == "ACK" {
                // Fragile slice
                return Err(Mpd::handle_error(&line));
            }
            let split: std::vec::Vec<&str> = line.split(":").collect();
            stats.insert(split[0].to_string(), split[1][1..].to_string());
        }
        Ok(stats)
    }
    pub fn clear_error(&mut self) -> Result<(), MpdError> {
        self.connection
            .write(b"clearerror\n")
            .expect("failed to write to MPD connection");

        let mut response = String::new();
        self.connection
            .read_to_string(&mut response)
            .expect("failed to read from MPD connection");

        if response == "OK" {
            Ok(())
        } else {
            Err(Mpd::handle_error(&response))
        }
    }
}
