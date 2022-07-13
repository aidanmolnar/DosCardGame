use dos_shared::cards::Card;


// Tracks the state of the game visible to the client
// TODO: Should this consider opponent card counts? Kind of unnecessary because that info would also be in tracker but this is info visible to you??
// TODO: Rename this to local player, local player state or something?
#[derive(Default)]
pub struct LocalPlayerState {
    your_cards: Vec<Card>,
    top_discard: Option<Card>,
}

// TODO: change directions, whose turn is it?
impl LocalPlayerState {

    // Adds a card to your hand.  Returns the hand position of the card
    pub fn receive_card(
        &mut self, 
        card: Card
    ) -> usize {
        let hand_position = self.your_cards.binary_search_by(|x| x.cmp(&card)).unwrap_or_else(|x| x);
        self.your_cards.insert(hand_position, card);

        hand_position
    }

    // Removes a card from your hand.  Returns the hand position the card had
    // Adds the card to the top of the discard pile
    // Panics if the card was not actually in your hand
    pub fn play_card(
        &mut self, 
        card: Card
    ) -> usize {
        let hand_position = self.your_cards.binary_search_by(|x| x.cmp(&card)).expect("Could not find card in hand!");
        let card = self.your_cards.remove(hand_position);
        self.add_to_discard_pile(card);

        hand_position
    }

    // Sets the top of the discard pile
    pub fn add_to_discard_pile(
        &mut self, 
        card: Card
    ) {
        self.top_discard = Some(card);
    }

    // Removes the discard pile for reshuffling
    pub fn clear_discard_pile(
        &mut self
    ) {
        self.top_discard = None;
    }

    // TODO: Get valid moves
}