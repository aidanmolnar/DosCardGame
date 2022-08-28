//TODO: This might not belong in this module

use super::TableArranger;

mod deck;
mod spawn;

pub use spawn::spawn_all_tables;
pub use deck::DeckBuilder;