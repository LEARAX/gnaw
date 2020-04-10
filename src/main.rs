use gnaw::Mpd;

fn main() {
    let mut mpd = Mpd::new(
        "127.0.0.1:6600"
            .parse()
            .expect("Failed to parse MPD address"),
    )
    .expect("Failed to connect to MPD");
    /*
     * if let Ok(current_song) = Mpd::current_song(&mut mpd) {
     *     println!("{:?}", current_song);
     * }
     */
}
