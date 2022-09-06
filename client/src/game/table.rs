use dos_shared::{table::{BasicTable, Table, CardWrapper}, cards::Card, messages::lobby::TableSnapshot};

use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct ClientTable (BasicTable<ClientItem>);

#[derive(Debug, Clone)]
pub struct ClientItem (pub Option<Card>);

impl CardWrapper for ClientItem {
    fn card(&self) ->&Card {
        self.0.as_ref().expect("Card must exist")
    }

    fn card_mut(&mut self) -> &mut Card {
        self.0.as_mut().expect("Card must exist")
    }
}

impl ClientTable {
    pub fn new_empty() -> Self {
        ClientTable (
            BasicTable(Vec::new())
        )
    }

    pub fn new_with_size(num_cards: usize) -> Self {
        ClientTable (
            BasicTable(vec![ClientItem(None); num_cards])
        )
    }

    pub fn new_with_cards(cards: Vec<Card>) -> Self {
        ClientTable (
            BasicTable(
                cards.iter().map(
                    |card|
                    ClientItem(Some(*card))
                ).collect()
            )
        )
    }

    pub fn from_snapshot(snapshot: TableSnapshot) -> Self {
        match snapshot {
            TableSnapshot::Known(cards) => Self::new_with_cards(cards),
            TableSnapshot::Unknown(num_cards) => Self::new_with_size(num_cards),
        }
    }
}

impl Table<ClientItem> for ClientTable {
    fn remove(
        &mut self,
        index: usize
    ) -> Option<ClientItem> {
        self.0.remove(index)
    }

    fn push(
        &mut self,
        item: ClientItem
    ) {
        self.0.push(item)
    }

    fn last(
        &self,
    ) -> Option<&ClientItem> {
        self.0.last()
    }

    fn last_mut(
        &mut self,
    ) -> Option<&mut ClientItem> {
        self.0.last_mut()
    }

    fn len(
        &self
    ) -> usize {
        self.0.len()
    }

    fn pop(
        &mut self
    ) -> Option<ClientItem> {
        self.0.pop()
    }

    fn get(
        &self,
        index: usize,
    ) -> Option<&ClientItem> {
        self.0.get(index)
    }
    
    fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut ClientItem> {
        self.0.get_mut(index)
    }

    fn shuffle(&mut self) {
        // Doesn't actually need to be shuffled.  Just wiped
    }
}

