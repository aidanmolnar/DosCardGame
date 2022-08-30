
use super::cards::{Card, CardColor};
use super::table::CardReference;

pub mod lobby {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub enum FromServer {
        CurrentPlayers { player_names: Vec<String>, turn_id: u8},
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
    use super::{Card, CardColor, CardReference};
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
    pub enum GameAction {
        DealIn ,
        PlayCard(CardReference),
        DrawCards,
        KeepStaging,
        DiscardWildColor(CardColor),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FromServer {
        pub action: GameAction,
        pub conditions: Vec<bool>,
        pub cards: Vec<Card>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FromClient(pub GameAction);

}
