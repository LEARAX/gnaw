#[derive(Debug)]
pub struct Mpd {
    pub connection: std::net::TcpStream,
}

impl Mpd {
    pub fn new(address: std::net::SocketAddr) -> Result<Mpd, &'static str> {
        if let Ok(stream) = std::net::TcpStream::connect(address) {
            // TODO: Validate MPD response
            Ok(Mpd { connection: stream })
        } else {
            return Err("Failed to connect to MPD!");
        }
    }
}
