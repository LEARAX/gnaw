use gnaw::Mpd;

fn main() {
    let mut mpd = Mpd::new(
        "127.0.0.1:6600"
            .parse()
            .expect("failed to parse MPD address"),
    )
    .expect("failed to connect to MPD");
    println!("MPD connection established!");
    println!("MPD: {:?}", mpd);
}
