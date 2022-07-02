use serde::{Serialize, Deserialize};

pub mod cards;
use cards::*;

pub const NUM_STARTING_CARDS: u8 = 10;
pub const DEFAULT_IP: &str = "localhost:3333";

#[derive(Serialize, Deserialize, Debug)]
pub struct Integer {
    x: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LobbyUpdateServer {
    CurrentPlayers { player_names: Vec<String>, turn_id: u8},
    //YouAreLobbyLeader,
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
    DealIn {your_cards: Vec<Card>, card_counts: Vec<u8>},
}