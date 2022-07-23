
use dos_shared::cards::Card;

use bevy::prelude::*;


#[derive(Default)]
pub struct UnsortedTable (Vec<Entity>);


impl UnsortedTable {
    pub fn insert(&mut self, entity: Entity) {
        self.0.push(entity);
    }

    pub fn remove(&mut self, index: Option<usize>) -> Entity {
        if let Some(index) = index {
            self.0.remove(index)
        } else {
            self.0.pop().expect("No cards left")
        }
    }

    // pub fn reshuffle(&mut self) {
    //     todo!();
    // }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn new(entities: Vec<Entity>) -> UnsortedTable {
        UnsortedTable(entities)
    }
}

#[derive(Default)]
pub struct SortedTable {
    cards: Vec<(Card, Entity)>,
    entities: Vec<Entity>
}

impl SortedTable {
    pub fn insert(&mut self, card: Card, entity: Entity) {
        let hand_position = self.cards.binary_search_by(|x| x.0.cmp(&card)).unwrap_or_else(|x| x);
        self.cards.insert(hand_position, (card, entity));

        self.entities.push(entity);
    }

    pub fn remove(&mut self, index: Option<usize>) -> Entity {

        let entity = if let Some(index) = index {
            self.entities.remove(index)
        } else {
            self.entities.pop().unwrap()
        };

        let index = self.cards.iter()
            .map(|x|x.1)
            .position(|e| e == entity).unwrap();

        self.cards.remove(index);

        entity
    }

    // pub fn reshuffle(&mut self) {
    //     todo!();
    // }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.cards.iter().map(|x|&x.1)
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}



#[derive(Component)]
pub enum ClientTable {
    UnsortedTable (UnsortedTable),
    SortedTable (SortedTable)
}

impl ClientTable {
    pub fn insert(&mut self, entity: Entity, card: Option<Card>) {
        match self {
            ClientTable::UnsortedTable(hand) => {
                hand.insert(entity)
            }
            ClientTable::SortedTable(hand) => {
                hand.insert(card.expect("Card value must be specified for sorted hand"), entity)
            }
        }
    }

    pub fn remove(&mut self, index: Option<usize>) -> Entity {
        match self {
            ClientTable::UnsortedTable(hand) => {
                hand.remove(index)
            }
            ClientTable::SortedTable(hand) => {
                hand.remove(index)
            }
        }
    }

    // pub fn reshuffle(&mut self) {
    //     todo!();
    // }

    pub fn iter(&'_ self) -> Box<dyn Iterator<Item = &Entity>+ '_> {
        match self {
            ClientTable::UnsortedTable(hand) => {
                Box::new(hand.iter())
            }
            ClientTable::SortedTable(hand) => {
                Box::new(hand.iter())
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ClientTable::UnsortedTable(hand) => {
                hand.len()
            }
            ClientTable::SortedTable(hand) => {
                hand.len()
            }
        }
    }
}