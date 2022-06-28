use serde::{Serialize, Deserialize};


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

