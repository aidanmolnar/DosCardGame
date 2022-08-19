use dos_shared::cards::Card;
use dos_shared::table::*;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

#[derive(Default, Component)]
pub struct ServerTable (pub Vec<Card>);


impl ServerTable {
    pub fn insert(&mut self, card: Card) {
        self.0.push(card);
    }

    pub fn remove(&mut self, index: Option<usize>) -> Card {
        if let Some(index) = index {
            self.0.remove(index)
        } else {
            self.0.pop().expect("No cards left")
        }
    }

    pub fn peek(&self, index: Option<usize>) -> Option<Card> {
        if let Some(index) = index {
            self.0.get(index).cloned()
        } else {
            self.0.last().cloned()
        }
    }

    // pub fn iter(&self) -> impl Iterator<Item = &Card> {
    //     self.0.iter()
    // }

    // pub fn len(&self) -> usize {
    //     self.0.len()
    // }

    // // Returns the hand position of the specified entity
    // pub fn get_index(&self, card: Card) -> Option<usize> {
    //     self.0.iter().position(|x| *x == card)
    // }

    pub fn new(cards: Vec<Card>) -> ServerTable {
        ServerTable(cards)
    }

    pub fn last(&self) -> Option<Card> {
        self.0.last().cloned()
    }
}


#[derive(SystemParam)]
pub struct CardTransferer<'w,'s> {
    map: Res<'w, TableMap>,
    tables: Query<'w, 's, &'static mut ServerTable>,
}

impl<'w,'s> CardTransferer<'w,'s> {
    pub fn find_table(
        &mut self,
        location: &Location,
    ) -> Mut<ServerTable> {
        let table_entity = self.map.0[location];
        self.tables.get_mut(table_entity).unwrap()
    }

    // TODO: this should really be called remove
    fn get (
        &mut self, 
        from: &CardReference
    ) -> Card {
        self.find_table(&from.location).remove(from.index)
    }

    fn insert (
        &mut self, 
        to: &CardReference, 
        card: Card, 
    ) {
        self.find_table(&to.location).insert(card);
    }

    pub fn peek (
        &mut self,
        from: &CardReference,
    ) -> Option<Card> {
        self.find_table(&from.location).peek(from.index)
    }

    pub fn peek_discard(&mut self) -> Option<Card> {
        self.find_table(
            &Location::DiscardPile,
        ).last()
    }

    pub fn peek_staging(&mut self) -> Option<Card> {
        self.find_table(
            &Location::Staging,
        ).last()
    }

    pub fn transfer (
        &mut self, 
        from: CardReference, 
        to: CardReference, 
    ) -> Card {
        let card = self.get(&from);
        self.insert(&to, card);

        card
    }
}
