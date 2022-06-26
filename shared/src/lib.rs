use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Integer {
    x: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LobbyUpdate {
    PlayerCount { players: Vec<String> }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientConnect {
    pub name: String
}