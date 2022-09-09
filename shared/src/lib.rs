pub mod cards;
pub mod messages;
pub mod table;
pub mod game_info;
pub mod dos_game;
pub mod table_map;
pub mod transfer;
pub mod channel_config;

pub use game_info::GameInfo;

pub const PROTOCOL_ID: u64 = 7;
pub const DEFAULT_IP: &str = "127.0.0.1:3333";

pub const NUM_STARTING_CARDS: u8 = 4;
pub const DECK_SIZE: usize = 108;
const CARDS_TO_RETAIN: usize = 9; 
// Cards to refrain from dealing
// 9 chosen so that at least one of them is not a wild card

/// Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    PostGame,
    Reconnect,
}

