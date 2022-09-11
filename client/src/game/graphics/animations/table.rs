use dos_shared::cards::Card;

use dos_shared::messages::lobby::TableSnapshot;
use dos_shared::table::{Table, BasicTable};

use bevy::prelude::{Entity, Component};

use crate::game::graphics::DeckBuilder;

// Animation representation of a card for tables and trackers
#[derive(Copy, Clone, Debug)]
pub struct AnimationItem(
    pub Option<Card>, // The card value (if known)
    pub Entity // The entity that represents the card (has sprite, pickable bundle, etc.)
);

// A table of cards used for animations and local client effects.
// Updates sequentially based on animation timings.
#[derive(Component, Debug, Clone)]
pub enum AnimationTable {
    Sorted(SortedTable),
    Unsorted(BasicTable<AnimationItem>),
}

// A table where cards are sorted by their value
#[derive(Debug, Clone)]
pub struct SortedTable {
    entities: BasicTable<Entity>, // Ordered by actual hand position
    cards: Vec<AnimationItem>, // A vec where an internal ordering by card value is maintaned
}

impl SortedTable {
    // Gets the position of the entity when sorted by card value
    fn sorted_index(&self, entity: Entity) -> Option<usize> {
        self.cards.iter()
        .map(|x|x.1)
        .position(|e| e == entity)
    }

    fn new_with_items(mut items: Vec<AnimationItem>) -> Self {
        let entities = items.iter().map(|item| item.1).collect::<Vec<_>>();
        items.sort_by(|a,b|a.0.cmp(&b.0));
        Self { 
            entities: BasicTable(entities), 
            cards: items 
        }
    }
}

impl AnimationTable {
    pub const fn new_sorted() -> Self {
        Self::Sorted(
        SortedTable {
            entities: BasicTable(Vec::new()),
            cards: Vec::new(),
        })
    }

    pub const fn new_unsorted() -> Self {
        Self::Unsorted(
            BasicTable(Vec::new())
        )
    }

    pub fn new_unsorted_with_items(items: Vec<AnimationItem>) -> Self {
        Self::Unsorted(
            BasicTable(items)
        )
    }

    pub fn unsorted_from_snapshot(deck_builder: &mut DeckBuilder, snapshot: TableSnapshot) -> Self {
        match snapshot {
            TableSnapshot::Known(cards) => {
                Self::new_unsorted_with_items(deck_builder.make_known_cards(cards))
            },
            TableSnapshot::Unknown(num_cards) => {
                Self::new_unsorted_with_items(deck_builder.make_unknown_cards(num_cards))
            },
        }

    }

    pub fn sorted_from_snapshot(deck_builder: &mut DeckBuilder, snapshot: TableSnapshot) -> Self {
        match snapshot {
            TableSnapshot::Known(cards) => 
                Self::Sorted(
                    SortedTable::new_with_items(
                        deck_builder.make_known_cards(cards)
                    )
                ),
            TableSnapshot::Unknown(_) => 
                panic!("Must be known values for table to be sorted!"),
        }
    }

    // TODO: Doesn't distinguish between table not having sorted index (i.e. unsorted) and entity not being in table
    pub fn sorted_index(&self, entity: Entity) -> Option<usize> {
        match self {
            Self::Sorted(table) => {
                table.sorted_index(entity)
            }
            Self::Unsorted(_) => {
                None
            }
        }
    }

    pub fn actual_index(&self, entity: Entity) -> Option<usize> {
        match self {
            Self::Sorted(table) => {
                table.entities.0.iter().position(|e| *e == entity)
            }
            Self::Unsorted(table) => {
                table.0.iter().position(|e| e.1 == entity)
            }
        }
    }

    // TODO: Doesn't distinguish between card value being unknown/face-down and card not being in table
    pub fn card(&self, entity: Entity) -> Option<Card> {
        match self {
            Self::Sorted(table) => {
                self.sorted_index(entity)
                .and_then(|index| table.cards[index].0)
            }
            Self::Unsorted(table) => {
                self.actual_index(entity)
                .and_then(|index| table.0[index].0)
            }
        }
    }

    // TODO: Maybe consider just defining iter on each sub-type instead of requiring dynamic dispatch
    pub fn iter_entities(&'_ self) -> Box<dyn Iterator<Item = &Entity> + '_> {
        match self {
            Self::Sorted(table) => {
                Box::new(table.cards.iter().map(|x| &x.1))
            }
            Self::Unsorted(table) => {
                Box::new(table.0.iter().map(|x| &x.1))
            }
        }
    }
}

impl Table<AnimationItem> for SortedTable {
    fn push(&mut self, item: AnimationItem) {
        let hand_position = self.cards.binary_search_by(|x| x.0.cmp(&item.0)).unwrap_or_else(|x| x);
        self.cards.insert(hand_position, item);
        self.entities.push(item.1);
    }

    fn remove(&mut self, index: usize) -> Option<AnimationItem> {
        if let Some(entity) = self.entities.remove(index) {
            // We know the entity has been inserted into the table if it was in entities
            Some(self.cards.remove(
                self.sorted_index(entity).unwrap()
            ))
        } else {
            None
        }
    }

    fn last(&self) -> Option<&AnimationItem> {
        self.entities.last()
        .map(|entity| 
            &self.cards[self.sorted_index(*entity).unwrap()]
        )
    }

    fn last_mut(
        &mut self,
    ) -> Option<&mut AnimationItem> {
        // TODO: Would need to make sure that cards remain sorted after update 
        panic!("Can't mutate sorted table.")
    }

    fn len(
        &self
    ) -> usize {
        self.entities.len()
    }

    fn pop(
        &mut self
    ) -> Option<AnimationItem> {
        if let Some(entity) = self.entities.pop() {
            // We know the entity has been inserted into the table if it was in entities
            Some(self.cards.remove(
                self.sorted_index(entity).unwrap()
            ))
        } else {
            None
        }
    }

    fn get(
        &self,
        index: usize,
    ) -> Option<&AnimationItem> {
        self.entities.get(index)
            .map(|entity| 
                &self.cards[self.sorted_index(*entity).unwrap()]
            )
    }

    fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut AnimationItem> {
        if let Some(entity) = self.entities.get(index) {
            // We know the entity has been inserted into the table if it was in entities
            let index =  self.sorted_index(*entity).unwrap();
            Some(&mut self.cards[
               index
            ])
        } else {
            None
        }
    }

    fn shuffle(&mut self) {
        panic!("Don't shuffle sorted tables")
    }

}

// Directly delegates trait to variants
impl Table<AnimationItem> for AnimationTable {
    fn remove(
        &mut self,
        index: usize
    ) -> Option<AnimationItem> {
        match self {
            Self::Sorted(table) => {table.remove(index)}
            Self::Unsorted(table) => {table.remove(index)}
        }
    }

    fn push(
        &mut self,
        item: AnimationItem
    ) {
        match self {
            Self::Sorted(table) => {table.push(item)}
            Self::Unsorted(table) => {table.push(item)}
        }
    }

    fn last(
        &self,
    ) -> Option<&AnimationItem> {
        match self {
            Self::Sorted(table) => {table.last()}
            Self::Unsorted(table) => {table.last()}
        }
    }

    fn last_mut(
        &mut self,
    ) -> Option<&mut AnimationItem> {
        match self {
            Self::Sorted(table) => {table.last_mut()}
            Self::Unsorted(table) => {table.last_mut()}
        }
    }

    fn len(
        &self
    ) -> usize {
        match self {
            Self::Sorted(table) => {table.len()}
            Self::Unsorted(table) => {table.len()}
        }
    }

    fn pop(
        &mut self
    ) -> Option<AnimationItem> {
        match self {
            Self::Sorted(table) => {table.pop()}
            Self::Unsorted(table) => {table.pop()}
        }
    }

    fn get(
        &self,
        index: usize,
    ) -> Option<&AnimationItem> {
        match self {
            Self::Sorted(table) => {table.get(index)}
            Self::Unsorted(table) => {table.get(index)}
        }
    }

    fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut AnimationItem> {
        match self {
            Self::Sorted(table) => {table.get_mut(index)}
            Self::Unsorted(table) => {table.get_mut(index)}
        }
    }

    fn shuffle(&mut self) {
        // Doesn't actually need to be shuffled.  Just wiped
    }
}