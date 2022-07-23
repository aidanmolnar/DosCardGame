
use std::net::TcpStream;

#[derive(Default, Debug)]
pub struct MultiplayerState {
    pub stream: Option<TcpStream>,
    pub player_names: Vec<String>,
    pub turn_id: usize,
}

impl MultiplayerState {
    pub fn set_connected(&mut self, stream: TcpStream) {
        self.stream = Some(stream);
    }

    pub fn set_disconnected(&mut self) {
        self.stream = None;
        self.player_names = Vec::new();
    }
}