use bevy::prelude::Entity;
use bevy::utils::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Default)]
pub struct TableMap (pub HashMap<Location, Entity>);

#[derive(Serialize, Deserialize)]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Location {
    Deck,
    DiscardPile,
    Hand {player_id: usize},
    Staging,
}

#[derive(Serialize, Deserialize)]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct CardReference {
    pub location: Location,
    pub hand_position: HandPosition,
}

#[derive(Serialize, Deserialize)]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum HandPosition {
    Last,
    Index(usize)
}

