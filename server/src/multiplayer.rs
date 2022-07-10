
use bevy::prelude::*;
use std::net::TcpStream;

#[derive(Default)]
pub struct MultiplayerState {
    pub streams: Vec<TcpStream>,
    pub player_names: Vec<String>,
}

#[derive(Component)]
pub struct TurnId {
    pub id: u8,
}

#[derive(Component)]
pub struct NetPlayer {
    pub name: String,
    pub stream: TcpStream,
}


