
use super::cards::Card;
use super::table::{CardTransfer, CardReference};

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
        StartGame,
    }
}

pub mod game {
    use super::{Card, CardTransfer, CardReference};

    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub enum FromServer {
        DealIn {your_cards: Vec<Card>, deck_size: usize, to_discard_pile: Vec<Card>},
        TransferCards(Vec<CardTransfer>),
        NextTurn,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum FromClient {
        PlayCard {card: CardReference},
        DrawCards,
        KeepStaging,
    }
}


