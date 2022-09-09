use crate::table::{Table, CardWrapper, Location, CardReference, HandPosition};
use crate::cards::{Card, CardType, CardColor};
use crate::transfer::CardTransfer;
use crate::{GameInfo, NUM_STARTING_CARDS, CARDS_TO_RETAIN};

pub const DECK_REFERENCE: CardReference = 
CardReference {
    location: Location::Deck, 
    hand_position: HandPosition::Last
};

pub const STAGING_REFERENCE: CardReference = 
CardReference{
    location: Location::Staging, 
    hand_position: 
    HandPosition::Last
};

pub const DISCARD_REFERENCE: CardReference = 
CardReference {
    location: Location::DiscardPile, 
    hand_position: HandPosition::Last
};

// TODO: Rename "Default" state to "Normal" or something similar to avoid confusion about it not being actual default state lol
#[derive(PartialEq, Eq, Default, Debug)]
pub enum TurnState {
    Default, 
    StagedCard, // Could be determined by checking staging table
    // Above is even more weird because clients are (currently) not synced with this behavior
    WildcardColorSelect, // Could be determined by checking discard pile top card
    ServerDealingStartingCards, #[default] // Could be determined by checking if there is a card on the discard pile
    Victory, // *TODO* No cards left and there is a card in the discard pile
}

pub trait DosGame<T: CardWrapper, U: Table<T> + 'static>: 
    CardTransfer<T, U> 
{
    fn get_turn_state(&self) -> TurnState {
        if let Some(discard_wrapper) = self.get(&DISCARD_REFERENCE) {
            let discard = discard_wrapper.card();

            if discard.color == CardColor::Wild {
                return TurnState::WildcardColorSelect;
            }

            if self.get(&STAGING_REFERENCE).is_some() {
                return TurnState::StagedCard;
            }

            TurnState::Default
        } else {
            TurnState::ServerDealingStartingCards
        }
    }

    fn deal_starting_cards(&mut self, deck_size: usize) {
        
        // Deal the player hand cards
        let mut count = 0;
        for _ in 0..NUM_STARTING_CARDS {
            for player_id in 0..self.game_info().num_players() {
                let to = CardReference{location: Location::Hand{player_id}, hand_position: HandPosition::Last};
                
                self.transfer(&DECK_REFERENCE, &to);

                // Exit before dealing last card so that it can be used for discard pile
                // TODO: this panics if num starting cards is very large
                if count >= deck_size - CARDS_TO_RETAIN {
                    return
                }
                count += 1;
            }
        }

        // Deal the discard pile cards
        loop {
            self.transfer(
                &DECK_REFERENCE,
                &DISCARD_REFERENCE,
            );
    
            match self.get(&DISCARD_REFERENCE).unwrap().card().ty {
                CardType::Wild | CardType::DrawFour => {continue},
                _=> {break}
            }
        }
    }

    fn play_card(
        & mut self,
        card_reference: &CardReference, 
    ) {
        self.transfer(
            card_reference, 
            &DISCARD_REFERENCE
        );

        let card = *self.get(&DISCARD_REFERENCE).expect("Discarded card must be visible for all").card();

        let hand = self.get_table(
            &Location::Hand { 
                player_id: self.game_info().current_turn() 
            }
        );

        //TODO: Check if a player has Dos or is out of cards
        if hand.len() == 2 {
            self.someone_has_two_cards(self.game_info().current_turn());
            // This is tricky, because its a time based action that can happen whenever
        } else if hand.len() == 0 {
            self.victory(self.game_info().current_turn());
            // Move to post game?
        }

        // TODO: If stacking is not allowed in the future (by rules options), draw-x cards should deal immediately and skip the next player
        // Note: Wild and DrawFour don't end a players turn because the player must select a color
        match card.ty {
            CardType::Basic(_) => {
                self.game_info_mut().next_turn();
            },
            CardType::Skip => {
                self.game_info_mut().skip_turn();
            },
            CardType::Reverse => {
                self.game_info_mut().switch_direction();

                #[allow(clippy::comparison_chain)] // More readable than match
                if self.game_info().num_players() == 2 {
                    self.game_info_mut().skip_turn();
                } else if self.game_info().num_players() > 2 {
                    self.game_info_mut().next_turn();
                }
            },
            CardType::DrawTwo => {
                if self.game_info().num_players() > 1 {
                    self.game_info_mut().stacked_draws += 2;
                }
                self.game_info_mut().next_turn();
            },
            CardType::Wild => {},
            CardType::DrawFour => {
                if self.game_info().num_players() > 1 {
                    self.game_info_mut().stacked_draws += 4;
                }
            },  
        }
        
    }

    fn validate_play_card(
        &self,
        player: usize,
        card_reference: &CardReference, // TODO: This should really only take a hand_position...
    ) -> bool {
        // Check that the player owns the card they are trying to play
        if let Location::Hand{player_id} = card_reference.location {
            if player_id != player {
                return false;
            }
        } else if card_reference.location != Location::Staging {
            return false
        }

        // Check that it is their turn and the turn state is correct for playing
        let turn_state = self.get_turn_state();
        if self.is_players_turn(player) && (turn_state == TurnState::Default || turn_state == TurnState::StagedCard) {

            // Check that the card actually exists
            self.get(card_reference)
            .map_or(false, |card_wrapper| {
                let discard = self.get(&DISCARD_REFERENCE).unwrap().card(); // Can unwrap because we already checked that a discarded card exists in get_turn_state

                // Check that the card is playable
                if self.game_info().stacked_draws > 0 {
                    // Must play a card that can stack.
                    is_valid_move(*card_wrapper.card(), *discard) && 
                    (card_wrapper.card().ty == CardType::DrawFour || card_wrapper.card().ty == CardType::DrawTwo)
                } else {
                    is_valid_move(*card_wrapper.card(), *discard)
                }
            })
        } else {
            false
        }
    }

    fn draw_cards(
        &mut self,
    ) {
        let condition = |game: &Self| {
            let discard = game.get(&DISCARD_REFERENCE).unwrap().card();
            let card = game.get(&DECK_REFERENCE).unwrap().card();
            is_valid_move(*card, *discard)
        };

        let to = CardReference{
            location: Location::Hand{
                player_id: self.game_info().current_turn()
            }, 
            hand_position: HandPosition::Last
        };

        // Handle case where draw-x cards have been played/stacked
        let mut stacked_draws = self.game_info().stacked_draws;
        if stacked_draws > 0 {
            while stacked_draws > 0 {
                // Reshuffle deck if needed
                if self.get_table(&Location::Deck).is_empty() {
                    if self.get_table(&Location::DiscardPile).len() == 1 {
                        // Failed to supply a needed card.
                        break
                    } 
                    
                    self.reshuffle();
                }

                stacked_draws -= 1;
                self.transfer(&DECK_REFERENCE, &to);
            }

            self.game_info_mut().stacked_draws = 0;
            self.game_info_mut().next_turn();
            return;
        }
        
        // Handle normal drawing case
        loop {
            // Reshuffle deck if needed
            if self.get_table(&Location::Deck).is_empty() {
                if self.get_table(&Location::DiscardPile).len() == 1 {
                    // Failed to supply a needed card.
                    self.game_info_mut().next_turn();
                    break
                }
                
                self.reshuffle();
            }

            if self.server_condition(condition) {
                self.transfer(&DECK_REFERENCE, &STAGING_REFERENCE);
                break
            }

            self.transfer(&DECK_REFERENCE, &to);
        }
    }

    fn validate_draw_cards(
        &self,
        player: usize,
    ) -> bool {
        self.is_players_turn(player) && self.get_turn_state() == TurnState::Default
    }
    
    fn keep_last_drawn_card(
        &mut self,
    ) {
        self.transfer(
            &CardReference{location: Location::Staging, hand_position: HandPosition::Last},
            &CardReference{location: Location::Hand{player_id: self.game_info().current_turn()}, hand_position: HandPosition::Last}
        );

        self.game_info_mut().next_turn();
    }

    fn validate_keep_last_drawn_card(
        &self,
        player: usize,
    ) -> bool {
        self.is_players_turn(player) && self.get_turn_state() == TurnState::StagedCard
    }

    fn declare_wildcard_color(
        &mut self,
        color: &CardColor,
    ) {
        let mut discard = *self.get(&DISCARD_REFERENCE).unwrap().card();
        discard.color = *color;
        self.set_discard_last(Some(discard));

        self.game_info_mut().next_turn();
    }

    fn validate_declare_wildcard_color(
        &self,
        player: usize,
        color: &CardColor,
    ) -> bool {
        if self.is_players_turn(player) && self.get_turn_state() == TurnState::WildcardColorSelect {
            *color != CardColor::Wild
        } else {
            false
        }
    }

    fn punish_missed_dos(&mut self, player: usize) {
        let to = CardReference{
            location: Location::Hand{
                player_id: player,
            }, 
            hand_position: HandPosition::Last
        };

        let mut punish_cards = 3;

        while punish_cards > 0 {
            // Reshuffle deck if needed
            if self.get_table(&Location::Deck).is_empty() {
                if self.get_table(&Location::DiscardPile).len() == 1 {
                    // Failed to supply a needed card.
                    self.game_info_mut().next_turn();
                    break
                }

                self.reshuffle();
            }

            punish_cards -= 1;
            self.transfer(&DECK_REFERENCE, &to);
        }
    }

    fn is_players_turn(
        &self, 
        player: usize
    ) -> bool {
       player == self.game_info().current_turn()
    }

    fn is_visible(
        &self,
        location: &Location,
        player: usize,
    ) -> bool {
        match location {
            Location::Deck => false,
            Location::DiscardPile => true,
            Location::Hand { player_id: hand_id } => {
                *hand_id == player 
            },
            Location::Staging => {
                player == self.game_info().current_turn()
            },
        }
    }

    // In some cases the visible state of the board is not enough for the client to reproduce an action
    // For example: when a different player asks to draw cards, a client wihtout visibility can't know how many the other is passed before they are able to play
    // This function checks if the condition is true on the server and increments a counter each check
    // On the client it decrements the counter until it is zero, which results in the same number of checks before the value is true
    // TODO: Allow more than one / nested conditions. Currently there can only be one for each message
    fn server_condition<F>(
        &mut self, 
        condition: F
    ) -> bool
    where F: Fn(&Self) -> bool;

    fn game_info(
        &self
    ) -> &GameInfo;

    fn game_info_mut(
        &mut self
    ) -> &mut GameInfo;

    fn set_discard_last(
        &mut self, 
        card: Option<Card>
    );

    fn transfer(
        &mut self,
        from: &CardReference,
        to: &CardReference,
    );

    fn reshuffle(&mut self);

    fn victory(&mut self, winner: usize);

    fn someone_has_two_cards(&mut self, player: usize);
}


fn is_valid_move(card: Card, discard_pile: Card) -> bool {
    card.ty == CardType::Wild || 
    card.ty == CardType::DrawFour ||
    card.color == discard_pile.color ||
    card.ty == discard_pile.ty
}
