use bevy::prelude::*;
use bevy::utils::HashMap;

// Shared

#[derive(Default)]
pub struct TableMap (pub HashMap<Location, Entity>);


#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Location {
    Deck,
    DiscardPile,
    Hand {player_id: usize}
}


pub struct CardReference {
    pub location: Location,
    pub index: Option<usize>,
}
