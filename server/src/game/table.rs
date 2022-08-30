use dos_shared::{cards::Card, transfer::{Table, BasicTable}};

use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct ServerTable(BasicTable<Card>);

impl ServerTable {
    pub fn new(cards: Vec<Card>) -> Self {
        ServerTable(
            BasicTable::<Card>(cards)
        )
    }
}

impl Default for ServerTable {
    fn default() -> Self {
        ServerTable(
            BasicTable::<Card>(Vec::new())
        )
    }
}

impl Table<Card> for ServerTable {
    fn remove(
        &mut self,
        index: usize
    ) -> Option<Card> {
        self.0.remove(index)
    }

    fn push(
        &mut self,
        item: Card
    ) {
        self.0.push(item)
    }

    fn last(
        &self,
    ) -> Option<&Card> {
        self.0.last()
    }

    fn last_mut(
        &mut self,
    ) -> Option<&mut Card> {
        self.0.last_mut()
    }

    fn len(
        &self
    ) -> usize {
        self.0.len()
    }

    fn pop(
        &mut self
    ) -> Option<Card> {
        self.0.pop()
    }

    fn get(
        &self,
        index: usize,
    ) -> Option<&Card> {
        self.0.get(index)
    }
    
    fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut Card> {
        self.0.get_mut(index)
    }

    fn shuffle(&mut self) {
        self.0.shuffle()
    }
}
