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
    match Mpd::current_song(&mut mpd) {
        Ok(current_song) => println!("{:?}", current_song),
        Err(e) => eprintln!("Error retrieving current song: {:?}", e),
    }
    match Mpd::status(&mut mpd) {
        Ok(status) => println!("{:?}", status),
        Err(e) => eprintln!("Error retrieving MPD status: {:?}", e),
    }
}
