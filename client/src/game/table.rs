use dos_shared::{transfer::{BasicTable, Table, CardWrapper}, cards::Card};

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
    pub fn new() -> Self {
        ClientTable (
            BasicTable(Vec::new())
        )
    }

    pub fn new_deck(num_cards: usize) -> Self {
        ClientTable (
            BasicTable(vec![ClientItem(None); num_cards])
        )
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
}

