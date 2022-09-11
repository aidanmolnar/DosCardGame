use dos_shared::{cards::Card, messages::game::FromServer};

use std::collections::VecDeque;

// A client cannot completely reproduce state from same set of actions, because it has incomplete information
// This stores additional information that the server shares on a per client basis so that each client can reproduce state.
#[derive(Default, Debug)]
pub struct ClientSyncer{
    cards: VecDeque<Card>, // Newly visible cards that the client couldn't see before
    conditions: VecDeque<bool>, // Conditions based on cards the client can't see
}

impl ClientSyncer {
    // Stores values from server message
    pub fn enque_all(&mut self, message: FromServer) {
        self.cards = VecDeque::from(message.cards);
        self.conditions = VecDeque::from(message.conditions);
    }
    
    pub fn deque_card(&mut self) -> Card {
        self.cards.pop_front().expect("No more card values stored")
    }

    pub fn deque_condition(&mut self) -> bool {
        self.conditions.pop_front().expect("No more condition values stored")
    }
}
