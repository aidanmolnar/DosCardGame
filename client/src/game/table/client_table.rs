
use dos_shared::cards::Card;
use dos_shared::table::{Location, CardReference};

use bevy::prelude::*;


#[derive(Component)]
pub enum ClientTable {
    UnsortedTable (UnsortedTable),
    SortedTable (SortedTable)
}

impl ClientTable {
    pub fn insert(&mut self, card: Option<Card>, entity: Entity) {
        match self {
            ClientTable::UnsortedTable(table) => {
                table.insert(card, entity)
            }
            ClientTable::SortedTable(table) => {
                table.insert(card.expect("Card value must be specified for sorted hand"), entity)
            }
        }
    }

    pub fn remove(&mut self, index: Option<usize>) -> Entity {
        match self {
            ClientTable::UnsortedTable(table) => {
                table.remove(index)
            }
            ClientTable::SortedTable(table) => {
                table.remove(index)
            }
        }
    }

    pub fn iter(&'_ self) -> Box<dyn Iterator<Item = &Entity>+ '_> {
        match self {
            ClientTable::UnsortedTable(table) => {
                Box::new(table.iter())
            }
            ClientTable::SortedTable(table) => {
                Box::new(table.iter())
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ClientTable::UnsortedTable(table) => {
                table.len()
            }
            ClientTable::SortedTable(table) => {
                table.len()
            }
        }
    }

    pub fn locate(&self, entity: Entity) -> Option<TableIndexData> {
        match self {
            ClientTable::UnsortedTable(table) => {
                table.get_index(entity)
                .map(|index| TableIndexData::Unsorted { hand_position: index })
            }
            ClientTable::SortedTable(table) => {
                table.get_actual_index(entity)
                .map(|index| {
                    let sorted_position = table.get_sorted_index(entity).unwrap();
                    TableIndexData::Sorted { 
                        hand_position: index,
                        sorted_position,
                        card_value: table.get_card_value(sorted_position)
                    }
                })
            }
        }
    }

    pub fn last(&self) -> Option<(Entity, Option<Card>)> {
        match self {
            ClientTable::UnsortedTable(table) => {
                table.last()
            }
            ClientTable::SortedTable(table) => {
                table.last()
            }
        }
    }
}

// TODO: rename or rework this
// Goals: 
// + Be able to get back additional info from sorted table that has more information about the card
// + Have response be easily readible
// Alternative: a struct with hand_position field and optional "sorted_data" that is usize, and card value 
//   (how would it be clear what exactly the usize is in this case)
// It's already a little unclear what the difference between sorted_position and hand_position is (TODO)
// TODO: UnsortedTable now has card value as well
pub enum TableIndexData {
    Sorted {
        hand_position: usize,
        sorted_position: usize,
        card_value: Card,
    },
    Unsorted {
        hand_position: usize
    },
}

impl TableIndexData {
    fn get_hand_position (&self) -> usize {
        match self {
            Self::Sorted{hand_position, ..} => *hand_position,
            Self::Unsorted{hand_position} => *hand_position,
        }
    }

    pub fn to_card_reference(&self, location: Location) -> CardReference {
        CardReference { location, index: Some(self.get_hand_position())}
    }

    pub fn get_card_value(&self) -> Option<Card> {
        match self {
            Self::Sorted{card_value, ..} => Some(*card_value),
            Self::Unsorted{..} => None,
        }
    }
}

#[derive(Default)]
pub struct UnsortedTable (Vec<(Entity, Option<Card>)>);


impl UnsortedTable {
    pub fn insert(&mut self, card: Option<Card>, entity: Entity) {
        self.0.push((entity, card));
    }

    pub fn remove(&mut self, index: Option<usize>) -> Entity {
        if let Some(index) = index {
            self.0.remove(index).0
        } else {
            self.0.pop().expect("No cards left").0
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter().map(|x| &x.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    // Returns the hand position of the specified entity
    pub fn get_index(&self, entity: Entity) -> Option<usize> {
        self.0.iter().position(|e| e.0 == entity)
    }

    pub fn new(entities: Vec<(Entity, Option<Card>)>) -> UnsortedTable {
        UnsortedTable(entities)
    }

    pub fn last(&self) -> Option<(Entity, Option<Card>)> {
        self.0.last().cloned()
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

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.cards.iter().map(|x|&x.1)
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn get_sorted_index(&self, entity: Entity) -> Option<usize> {
        self.cards.iter().position(|e| e.1 == entity)
    }

    pub fn get_actual_index(&self, entity: Entity) -> Option<usize> {
        self.entities.iter().position(|e| *e == entity)
    }

    pub fn get_card_value(&self, sorted_index: usize) -> Card {
        self.cards[sorted_index].0
    }

    pub fn last(&self) -> Option<(Entity, Option<Card>)> {
        if let Some(entity) = self.entities.last() {
            let card_value = self.get_card_value(
                self.get_sorted_index(*entity).unwrap()
            );
            return Some((*entity, Some(card_value)))
        }
        
        None
    }
}
