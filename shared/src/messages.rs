
use super::cards::Card;

pub mod lobby {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub enum FromServer {
        CurrentPlayers { player_names: Vec<String>, turn_id: u8},
        //YouAreLobbyLeader,
        Disconnect,
        StartGame,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum FromClient {
        Connect {name: String},
        //Disconnect,
        StartGame,
    }
}

pub mod game {
    use serde::{Serialize, Deserialize};
    use super::Card;

    #[derive(Serialize, Deserialize, Debug)]
    pub enum FromServer {
        DealIn {your_cards: Vec<Card>, deck_size: usize},
        YourTurn,
    }
}


