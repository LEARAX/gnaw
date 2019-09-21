use std::io::prelude::*;
use std::io::BufReader;

pub struct Mpd {
    pub connection: std::net::TcpStream,
    pub version: String,
}

pub struct Song {
    pub title: String,
    pub album: String,
    pub artist: String,
    pub duration: f64,
    pub elapsed: f64,
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
        let mut song_title = None;
        let mut song_album = None;
        let mut song_artist = None;
        let mut song_duration = None;
        for line in reader.lines() {
            if let Ok(data) = line.as_ref() {
                if data == "OK" {
                    break;
                } else if Some("Title:") == data.get(0..6) {
                    song_title = Some(data[7..].to_string());
                } else if Some("Album:") == data.get(0..6) {
                    song_album = Some(data[7..].to_string());
                } else if Some("Artist:") == data.get(0..7) {
                    song_artist = Some(data[8..].to_string());
                } else if Some("duration:") == data.get(0..9) {
                    let parsed_duration = data[10..].parse::<f64>();
                    if let Ok(exp_duration) = parsed_duration {
                        song_duration = Some(exp_duration);
                    } else {
                        song_duration = None;
                    }
                }
            } else {
                return Err("failed to read song data from MPD");
            };
        }

        let mut song_elapsed = None;
        self.connection
            .write(b"status\n")
            .expect("failed to write to MPD connection");
        let reader = BufReader::new(&self.connection);
        for line in reader.lines() {
            if let Ok(data) = line.as_ref() {
                if data == "OK" {
                    break;
                } else if Some("elapsed:") == data.get(0..8) {
                    let parsed_elapsed = data[9..].parse::<f64>();
                    if let Ok(exp_elapsed) = parsed_elapsed {
                        song_elapsed = Some(exp_elapsed);
                    } else {
                        song_elapsed = None;
                    }
                }
            }
        }

        if song_title.is_some()
            && song_album.is_some()
            && song_artist.is_some()
            && song_duration.is_some()
            && song_elapsed.is_some()
        {
            Ok(Song {
                title: song_title.unwrap(),
                album: song_album.unwrap(),
                artist: song_artist.unwrap(),
                duration: song_duration.unwrap(),
                elapsed: song_elapsed.unwrap(),
            })
        } else {
            Err("song data not found")
        }
    }
}
