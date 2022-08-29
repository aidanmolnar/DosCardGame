use dos_shared::cards::Card;

use std::collections::VecDeque;


#[derive(Default, Debug)]
pub struct ClientSyncer{
    cards: VecDeque<Card>, 
    pub condition_counter: usize,
}

impl ClientSyncer {
    pub fn enque(&mut self, card: Card) {
        self.cards.push_back(card);
    }

    pub (crate) fn deque(&mut self) -> Card {
        self.cards.pop_front().expect("No more card values stored")
    }
}
