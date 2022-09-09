
use super::cards::{Card, CardColor};
use super::table::CardReference;

pub mod lobby {
    use bevy::utils::HashMap;
    use serde::{Serialize, Deserialize};

    use crate::{table::Location, GameInfo, cards::Card};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum FromServer {
        CurrentPlayers { player_names: Vec<String>, turn_id: usize},
        StartGame,
        Reject{reason: String},
        Reconnect (GameSnapshot)
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum TableSnapshot {
        Known(Vec<Card>),
        Unknown(usize),
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct GameSnapshot {
        pub tables: HashMap<Location, TableSnapshot>,
        pub game_info: GameInfo,
        pub dos: Option<usize>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum FromClient {
        Connect {name: String},
        StartGame,
    }

    
}

pub mod game {
    use super::{Card, CardColor, CardReference};
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum GameAction {
        DealIn,
        PlayCard(CardReference),
        DrawCards,
        KeepStaging,
        DiscardWildColor(CardColor),
        CallDos(Option<CallDosInfo>), // Server includes info, client does not.
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct CallDosInfo {
        pub player: usize,
        pub caller: usize, 
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FromServer {
        pub action: GameAction,
        pub conditions: Vec<bool>,
        pub cards: Vec<Card>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct FromClient(pub GameAction);

    

}
