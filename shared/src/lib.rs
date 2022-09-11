#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::needless_pass_by_value)] // Bevy systems require resources to be passed by value
#![allow(clippy::use_self)]

pub mod cards;
pub mod messages;
pub mod table;
pub mod dos_game;
pub mod table_map;
pub mod transfer;
pub mod net_config;
mod game_info;

pub use game_info::GameInfo;

pub const NUM_STARTING_CARDS: u8 = 7;
pub const DECK_SIZE: usize = 108;

/// Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    PostGame,
    Reconnect,
}

