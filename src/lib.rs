use std::io::prelude::*;
use std::io::BufReader;

pub struct Mpd {
    pub connection: std::net::TcpStream,
    pub reader: std::io::BufReader<std::net::TcpStream>,
    pub version: String,
}

pub type Queue = Vec<Song>;

pub struct Song {
    pub file: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub album_artist_sort: Option<String>,
    pub artist: Option<String>,
    pub artist_sort: Option<String>,
    pub label: Option<String>,
    pub genre: Option<String>,
    pub title: Option<String>,
    pub track: Option<usize>,
    pub duration: Option<f64>,
    pub position: Option<usize>,
    pub id: Option<usize>,
}

impl Mpd {
    pub fn new(address: std::net::SocketAddr) -> Result<Mpd, &'static str> {
        if let Ok(stream) = std::net::TcpStream::connect(address) {
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut buffer = String::new();
            reader
                .read_line(&mut buffer)
                .expect("failed to read initial response from MPD");
            if &buffer[0..6] == "OK MPD" {
                Ok(Mpd {
                    connection: stream,
                    reader: reader,
                    version: buffer[7..buffer.len() - 1].to_string(),
                })
            } else {
                Err("MPD returned an inappropriate response")
            }
        } else {
            Err("failed to connect to MPD")
        }
    }

    fn parse_song_data(reader: std::io::BufReader<std::net::TcpStream>) -> Result<Song, &'static str> {
        let mut song = Song {
            file: None,
            album: None,
            album_artist: None,
            album_artist_sort: None,
            artist: None,
            artist_sort: None,
            label: None,
            genre: None,
            title: None,
            track: None,
            duration: None,
            position: None,
            id: None,
        };
        for line in reader.lines() {
            if let Ok(data) = &line.as_ref() {
                if data == &"OK" {
                    break;
                } else if Some("file:") == data.get(0..5) {
                    song.file = Some(data[6..].to_string());
                } else if Some("Album:") == data.get(0..6) {
                    song.album = Some(data[7..].to_string());
                } else if Some("AlbumArtist:") == data.get(0..12) {
                    song.album_artist = Some(data[13..].to_string());
                } else if Some("AlbumArtistSort:") == data.get(0..16) {
                    song.album_artist_sort = Some(data[17..].to_string());
                } else if Some("Artist:") == data.get(0..7) {
                    song.artist = Some(data[8..].to_string());
                } else if Some("ArtistSort:") == data.get(0..11) {
                    song.artist_sort = Some(data[12..].to_string());
                } else if Some("Label:") == data.get(0..6) {
                    song.label = Some(data[7..].to_string());
                } else if Some("Genre:") == data.get(0..6) {
                    song.genre = Some(data[7..].to_string());
                } else if Some("Title:") == data.get(0..6) {
                    song.title = Some(data[7..].to_string());
                } else if Some("Track:") == data.get(0..6) {
                    let parsed_track = data[7..].parse::<usize>();
                    if let Ok(exp_track) = parsed_track {
                        song.track = Some(exp_track);
                    } else {
                        song.track = None;
                    }
                    song.label = Some(data[7..].to_string());
                } else if Some("duration:") == data.get(0..9) {
                    let parsed_duration = data[10..].parse::<f64>();
                    if let Ok(exp_duration) = parsed_duration {
                        song.duration = Some(exp_duration);
                    } else {
                        song.duration = None;
                    }
                } else if Some("Pos:") == data.get(0..4) {
                    let parsed_position = data[5..].parse::<usize>();
                    if let Ok(exp_position) = parsed_position {
                        song.position = Some(exp_position);
                    } else {
                        song.position = None;
                    }
                } else if Some("Id:") == data.get(0..3) {
                    let parsed_id = data[4..].parse::<usize>();
                    if let Ok(exp_id) = parsed_id {
                        song.id = Some(exp_id);
                    } else {
                        song.id = None;
                    }
                    break;
                }
            } else {
                return Err("failed to read song data from MPD");
            };
        }
        return Ok(song);
    }

    pub fn fetch_current_song(&mut self) -> Result<(Song, Option<f64>), &'static str> {
        let mut song_elapsed = None;
        self.connection
            .write(b"status\n")
            .expect("failed to write to MPD connection");
        for line in self.reader.lines() {
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
            } else {
                return Err("failed to parse status data");
            }
        }
        self.connection
            .write(b"currentsong\n")
            .expect("failed to write to MPD connection");
        if let Ok(song) = Mpd::parse_song_data(self.reader) {
            return Ok((song, song_elapsed));
        } else {
            return Err("failed to parse song data")
        }
    }

    pub fn fetch_queue(&mut self) -> Result<Queue, &'static str> {
        let mut queue = Vec::new();
        self.connection
            .write(b"playlistinfo\n")
            .expect("failed to write to MPD connection");
        while self.reader.buffer() != &[] {
            queue.push(Mpd::parse_song_data(self.reader).unwrap());
        }
        Ok(queue)
    }
}
