fn main() {
    if let Ok(mpd) = gnaw::Mpd::new("127.0.0.1:6600".parse().unwrap()) {
        println!("{:#?}", mpd);
    } else {
        panic!("Failed to connect to mpd!")
    }
}
