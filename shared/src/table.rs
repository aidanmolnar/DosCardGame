use bevy::prelude::*;
use bevy::utils::HashMap;

use serde::{Serialize, Deserialize};

// Shared


#[derive(Default)]
pub struct TableMap (pub HashMap<Location, Entity>);

#[derive(Serialize, Deserialize)]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Location {
    Deck,
    DiscardPile,
    Hand {player_id: usize}
}

#[derive(Serialize, Deserialize)]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct CardReference {
    pub location: Location,
    pub index: Option<usize>,
}
