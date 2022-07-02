
use std::net::TcpStream;

#[derive(Default, Debug)]
pub struct MultiplayerState {
    pub stream: Option<TcpStream>,
    pub player_names: Vec<String>,
    pub turn_id: u8,
}
