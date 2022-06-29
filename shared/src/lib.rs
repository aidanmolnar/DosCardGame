use serde::{Serialize, Deserialize};

pub mod cards;
use cards::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Integer {
    x: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LobbyUpdateServer {
    CurrentPlayers { player_names: Vec<String> },
    YouAreLobbyLeader,
    Disconnect,
    StartGame,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LobbyUpdateClient {
    Connect {name: String},
    Disconnect,
    StartGame,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GameUpdateServer {
    DealIn {cards: Vec<Card>},
}