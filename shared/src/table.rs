use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Serialize, Deserialize};

use super::cards::Card;

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

pub trait Table<T> {
    fn remove(
        &mut self,
        index: usize
    ) -> Option<T>;

    fn get(
        &self,
        index: usize,
    ) -> Option<&T>;

    fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut T>;

    fn push(
        &mut self,
        item: T
    );

    fn last(
        &self,
    ) -> Option<&T>;

    fn last_mut(
        &mut self,
    ) -> Option<&mut T>;

    fn len(
        &self
    ) -> usize;

    fn is_empty(
        &self
    ) -> bool {
        self.len() == 0
    }

    fn pop(
        &mut self
    ) -> Option<T>;

    fn shuffle(&mut self);
}

pub trait CardWrapper {
    fn card(&self) ->&Card;
    fn card_mut(&mut self) -> &mut Card;
}

impl CardWrapper for Card {
    fn card(&self) ->&Card {
        self
    }

    fn card_mut(&mut self) -> &mut Card {
        self
    }
}


#[derive(Debug, Clone)]
pub struct BasicTable<T> (pub Vec<T>);

impl<T: std::fmt::Debug> Table<T> for BasicTable<T> {
    fn remove(
        &mut self,
        index: usize
    ) -> Option<T> {
        if index < self.0.len() {
            Some(self.0.remove(index))
        } else {
            None
        }
    }

    fn get(
        &self,
        index: usize,
    ) -> Option<&T> {
        self.0.get(index)
    }

    fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut T> {
        self.0.get_mut(index)
    }

    fn push(
        &mut self,
        item: T
    ) {
        self.0.push(item)
    }

    fn last(
        &self,
    ) -> Option<&T> {
        self.0.last()
    }

    fn last_mut(
        &mut self,
    ) -> Option<&mut T> {
        self.0.last_mut()
    }

    fn len(
        &self
    ) -> usize {
        self.0.len()
    }

    fn pop(
        &mut self
    ) -> Option<T> {
        self.0.pop()
    }

    fn shuffle(&mut self) {
        self.0.shuffle(&mut thread_rng());
    }
    
}