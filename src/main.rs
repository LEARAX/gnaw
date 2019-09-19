use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    if let Ok(mut mpd) = gnaw::Mpd::new("127.0.0.1:6600".parse().unwrap()) {
        let mut reader = BufReader::new(mpd.connection);
        let mut buffer = String::new();
        if let Ok(_) = reader.read_line(&mut buffer) {
            println!("Buffer: {:#?}", buffer)
        } else{
            panic!("Failed to read version information from MPD!")
        }
    } else {
        panic!("Failed to connect to mpd!")
    }
}
