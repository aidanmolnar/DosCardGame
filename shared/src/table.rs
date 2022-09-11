use super::cards::Card;

use rand::{seq::SliceRandom, thread_rng};
use serde::{Serialize, Deserialize};

// A table is a collection of cards.
// Each table has a unique Location.
// All cards are always in exactly one table.
// The client and server have the same table locations.
// Each card has a unique HandPosition in a table.  
// The positions are integers between 0 and the number of cards in the table.

// Cards are represented differently on the client and the server.
// On the server, cards are just the card value.
// The client has effectively two different states: an animation state, and a game state.
// The game state is updated immediately when an action is executed either from player input or server message.
// The animation state lags behind the game state. Card transfers are delayed so that they are not all executed at the same time.
// The client game state represents cards as Option<Card>. If the card is none, that means the client cannot see it (ex. it is in another hand or in the deck)
// The client animation state represents cards as (Option<Card>, Entity), where the Entity is a bevy entitiy attached to the visual representation of the card.

// Unique table position
#[derive(Serialize, Deserialize)]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Location {
    Deck,
    DiscardPile,
    Hand {player_id: usize},
    Staging,
}

// The table the card is in plus it's position in the table
#[derive(Serialize, Deserialize)]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct CardReference {
    pub location: Location,
    pub hand_position: HandPosition,
}

#[derive(Serialize, Deserialize)]
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum HandPosition {
    Last, // The last card in the table
    Index(usize) // The actual index of the card in the table
}

// Methods for interacting with a table of cards
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

// Used to convert client card representations to the underlying card value
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

// A thin wrapper over vec that implements Table trait.
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
        self.0.push(item);
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