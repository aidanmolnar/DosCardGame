use dos_shared::cards::Card;

use std::collections::VecDeque;


#[derive(Default, Debug)]
pub struct MemorizedCards(VecDeque<Card>, pub usize);

impl MemorizedCards {
    pub fn enque(&mut self, card: Card) {
        self.0.push_back(card);
    }

    pub (crate) fn deque(&mut self) -> Card {
        self.0.pop_front().expect("No more card values stored")
    }
}
