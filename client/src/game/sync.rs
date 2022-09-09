use dos_shared::{cards::Card, messages::game::FromServer};

use std::collections::VecDeque;


#[derive(Default, Debug)]
pub struct ClientSyncer{
    cards: VecDeque<Card>, 
    conditions: VecDeque<bool>,
}

impl ClientSyncer {
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
