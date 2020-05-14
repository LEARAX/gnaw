use gnaw::Mpd;

fn main() {
    let mut mpd = Mpd::new(
        "127.0.0.1:6600"
            .parse()
            .expect("failed to parse MPD address"),
    )
    .expect("failed to connect to MPD");
    println!("MPD connection established!");
    let current_song = Mpd::current_song(&mut mpd);
    if let Ok(current_song) = current_song {
        println!("{:?}", current_song);
    } else {
        eprintln!("Error retrieving current song: {:?}", current_song.unwrap());
    }
}
