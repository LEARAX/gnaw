fn main() {
    let mpd = gnaw::mpd_connect("127.0.0.1:6600".parse().unwrap());
    println!("{:#?}", mpd);
}
