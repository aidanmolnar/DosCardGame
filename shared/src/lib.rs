pub mod cards;
pub mod messages;
pub mod table;
mod test;

use cards::*;

pub const NUM_STARTING_CARDS: u8 = 20;
pub const DEFAULT_IP: &str = "localhost:3333";

const CARDS_TO_RETAIN: usize = 9; 
// Cards to refrain from dealing
// 9 chosen so that at least one of them is not a wild card

// TODO: Move to shared
/// Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
}

// Function that defines the pattern of dealing out cards
pub fn deal_cards<F: FnMut(usize)>(
    num_players: usize, 
    deck_size: usize,
    mut f: F,
) {
    let mut count = 0;

    for _ in 0..NUM_STARTING_CARDS {
        for player_id in 0..num_players {
            f(player_id); // do something

            // Exit before dealing last card so that it can be used for discard pile
            // TODO: this panics is num starting cards is very large
            if count >= deck_size - CARDS_TO_RETAIN {
                return
            }
            count += 1;
        }
    }
}

pub fn valid_move(card: Card, discard_pile: Card) -> bool {
    card.ty == CardType::Wild || 
    card.ty == CardType::DrawFour ||
    card.color == discard_pile.color ||
    card.ty == discard_pile.ty
}