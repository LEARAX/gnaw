use std::io::prelude::*;
use std::io::BufReader;

pub struct Mpd {
    pub connection: std::net::TcpStream,
    pub version: String,
}

#[derive(Debug)]
pub struct Song {
    pub title: String,
    pub album: String,
    pub artist: String,
    pub duration: f64,
}

impl Mpd {
    pub fn new(address: std::net::SocketAddr) -> Result<Mpd, &'static str> {
        if let Ok(stream) = std::net::TcpStream::connect(address) {
            let mut reader = BufReader::new(&stream);
            let mut buffer = String::new();
            reader
                .read_line(&mut buffer)
                .expect("failed to read initial response from MPD");
            if &buffer[0..6] == "OK MPD" {
                buffer.pop();
                Ok(Mpd {
                    connection: stream,
                    version: buffer[7..].to_string(),
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
        let mut song_title = String::new();
        let mut song_album = String::new();
        let mut song_artist = String::new();
        let mut song_duration: f64 = 0.0;
        for line in reader.lines() {
            if line.as_ref().unwrap().len() >= 7 {
                if let Ok(data) = line.as_ref() {
                    if let Some(start) = data[..].get(0..9) {
                        if &start[0..6] == "Title:" {
                            song_title = data[7..].to_string();
                        } else if &start[0..6] == "Album:" {
                            song_album = data[7..].to_string();
                        } else if &start[0..7] == "Artist:" {
                            song_artist = data[8..].to_string();
                        } else if &start[0..9] == "duration:" {
                            song_duration = data[10..].parse::<f64>().unwrap();
                        };
                    };
                } else {
                    return Err("failed to read data from MPD");
                };
            } else if let Ok(data) = line.as_ref() {
                if data == "OK" {
                    break;
                }
            }
        }
        if song_title == String::new() {
            return Err("failed to locate title string in MPD response");
        };
        if song_album == String::new() {
            return Err("failed to locate album string in MPD response");
        };
        if song_artist == String::new() {
            return Err("failed to locate artist string in MPD response");
        };
        if song_duration == 0.0 {
            return Err("failed to locate duration string in MPD response");
        };

        Ok(Song {
            title: song_title,
            album: song_album,
            artist: song_artist,
            duration: song_duration,
        })
    }
}
