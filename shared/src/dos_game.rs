use crate::transfer::{Table, CardTracker, CardWrapper};
use crate::table::{Location, CardReference, HandPosition};
use crate::cards::{Card, CardType, CardColor};
use crate::{GameInfo, NUM_STARTING_CARDS, CARDS_TO_RETAIN};

// TODO: Trait that describes all server/client actions that advance the game
// TODO: Figure out how to handle special card effects
// TODO: How do we validate client actions?
//       Can they play that card?  Did they delcare a valid wildcard color?
//       These functions could just return a bool if it was valid or not?
// TODO: How do we handle card effects?

// TODO: Weird states:
// * Client has selected play_cards but has not received cards from server
// * Client has played a draw_two but cards have not been dealt yet
// * Client has asked to draw cards and is deciding whether to play staged card or not
// * Client has played a wild card and has not selected color yet

const DECK_REFERENCE: CardReference = 
CardReference {
    location: Location::Deck, 
    hand_position: HandPosition::Last
};

const STAGING_REFERENCE: CardReference = 
CardReference{
    location: Location::Staging, 
    hand_position: 
    HandPosition::Last
};

const DISCARD_REFERENCE: CardReference = 
CardReference {
    location: Location::DiscardPile, 
    hand_position: HandPosition::Last
};

// TODO: Rename default state to "Normal" or something similar to avoid confusion about it not being actual default state lol
#[derive(PartialEq, Eq, Default)]
pub enum TurnState {
    Default, 
    StagedCard, // Could be determined by checking staging table
    // Above is even more weird because clients are (currently) not synced with this behavior
    WildcardColorSelect, // Could be determined by checking discard pile top card
    ServerDealingStartingCards, #[default] // Could be determined by checking if there is a card on the discard pile
    Victory, // *TODO* No cards left and there is a card in the discard pile
}

pub trait DosGame<T: CardWrapper + std::fmt::Debug, U: Table<T> + std::fmt::Debug + 'static>: 
    CardTracker<T, U> 
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
        let condition = |game: &Self| {
            match game.get(&DISCARD_REFERENCE).unwrap().card().ty {
                CardType::Wild => {false},
                CardType::DrawFour => {false}
                _=> {true}
            }
        };
        
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

        loop {
            self.transfer(
                &DECK_REFERENCE,
                &DISCARD_REFERENCE,
            );
    
            if self.server_condition(condition) {
                break;
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

        let card = self.get(&DISCARD_REFERENCE).expect("Discarded card must be visible for all").card();

        // TODO: Handle card effects: ex. draw cards, skip, reverse, etc.

        if card.color != CardColor::Wild {
            self.game_info_mut().next_turn();
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
            if let Some(card_wrapper) = self.get(card_reference) {
                let discard = self.get(&DISCARD_REFERENCE).unwrap().card(); // Can unwrap because we already checked that a discarded card exists in get_turn_state

                // Check that the card is playable
                is_valid_move(card_wrapper.card(), discard)
            } else {
                false
            }
        } else {
            false
        }
    }

    fn draw_cards(
        &mut self,
    ) {
        let condition = |game: &Self| {
            let discard = game.get(&DISCARD_REFERENCE).unwrap().card();
            let card = game.get(&DECK_REFERENCE).expect("Deck out of cards").card();
            is_valid_move(card, discard)
        };

        let to = CardReference{location: Location::Hand{player_id: self.game_info().current_turn()}, hand_position: HandPosition::Last};

        loop {
            // TODO: We can be sure the deck will have cards because card transferer will automatically reshuffle pile
        
            if self.server_condition(condition) {
                self.transfer(&DECK_REFERENCE, &STAGING_REFERENCE);
                break
            } else {
                self.transfer(&DECK_REFERENCE, &to);
            }
        }
    }

    fn server_condition<F>(&mut self, condition: F) -> bool
    where F: Fn(&Self) -> bool;

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

    fn game_info(&self) -> &GameInfo;
    fn game_info_mut(&mut self) -> &mut GameInfo;

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

    fn is_players_turn(
        &self, 
        player: usize
    ) -> bool {
       player == self.game_info().current_turn()
    }

    fn set_discard_last(
        &mut self, 
        card: Option<Card>
    );

    fn transfer(
        &mut self,
        from: &CardReference,
        to: &CardReference,
    );
}


fn is_valid_move(card: &Card, discard_pile: &Card) -> bool {
    card.ty == CardType::Wild || 
    card.ty == CardType::DrawFour ||
    card.color == discard_pile.color ||
    card.ty == discard_pile.ty
}
