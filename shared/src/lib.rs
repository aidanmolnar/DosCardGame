use serde::{Serialize, Deserialize};
use std::io;

#[derive(Serialize, Deserialize, Debug)]
pub struct Integer {
    x: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LobbyUpdateServer {
    CurrentPlayers { player_names: Vec<String> },
    YouAreLobbyLeader,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LobbyUpdateClient {
    Connect {name: String},
    Disconnect,
    StartGame,
}

pub fn handle_error(e: Box<bincode::ErrorKind>) {
    match *e {
        bincode::ErrorKind::Io(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
        _ => {
            println!("Failed to receive data: {}", e);
        }
    }
}