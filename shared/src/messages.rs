use super::{
    cards::{Card, CardColor}, 
    table::CardReference
};

// Messages sent over lobby channel
pub mod lobby {
    use crate::{table::Location, GameInfo, cards::Card};

    use bevy::utils::HashMap;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum FromServer {
        CurrentPlayers {player_names: Vec<String>, turn_id: usize},
        StartGame,
        Reject{reason: String},
        Reconnect (GameSnapshot)
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum TableSnapshot {
        Known(Vec<Card>), // Player can see cards in the table
        Unknown(usize), // Player cannot see cards in the table
    }

    // Complete game state for reconnecting to an ongoing game
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct GameSnapshot {
        pub tables: HashMap<Location, TableSnapshot>, // All information about card positions and values
        pub game_info: GameInfo,
        pub dos: Option<usize>, // Whether someone can "call dos"
    }


    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum FromClient {
        StartGame, 
        // TODO: Additional variants in future when setting custom rules?
    }
}

// Messages sent over game channel
pub mod game {
    use super::{Card, CardColor, CardReference};
    use serde::{Serialize, Deserialize};

    // Any action that advances the game state
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
        pub player: usize, // Player who has two cards remaining
        pub caller: usize, // Player who called that someone has two cards remaning
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FromServer {
        pub action: GameAction,
        pub conditions: Vec<bool>, // Game logic conditions that client can't know because not all cards are visible to the client
        pub cards: Vec<Card>, // Cards that have just become visible to client
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct FromClient(pub GameAction);
}
