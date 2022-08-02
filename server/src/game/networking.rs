use dos_shared::messages::game::*;
use dos_shared::cards::{Card, CardType, new_deck};
use super::multiplayer::{NetPlayer, Agent};

use bevy::prelude::*;


// TODO: break up into smaller pieces
pub fn enter_game_system(
    mut commands: Commands,
    query: Query<(&NetPlayer, &Agent)>,
) {
    let mut deck = new_deck(); // Get a standard shuffled deck of cards
    let deck_size = deck.len();

    // Create a vector of "hands" of cards
    let mut hands = Vec::new();
    for _ in 0..query.iter().len() {
        hands.push(Vec::new());
    }

    dos_shared::deal_cards(
        hands.len(),
        deck.len(),
        |player_id: usize| {
            hands[player_id]
                .push(deck.pop().unwrap());
        },
    );

    // Discards cards until a non wild one is found
    let mut discard_pile = Vec::new();
    loop {
        let card = deck.pop().unwrap();
        discard_pile.push(card);

        match card.ty {
            CardType::Wild => {continue},
            CardType::DrawFour => {continue}
            _=> {break}
        }
    }
    
    // TODO: there is probably a better/more functional way to do this that doesn't require cloning the hands
    for (i, (player, agent)) in query.iter().enumerate() {
        let hand = &hands[i];

        if let Err(e) = bincode::serialize_into(
            &player.stream, 
            &FromServer::DealIn{
                your_cards: hand.clone(),
                deck_size,
                to_discard_pile: discard_pile.clone(),
            }
        ) {
            println!("Deal in message failed to send {e}");
            // TODO: might need to disconnect client here, or return to lobby?
        }

        if agent.turn_id == 0 {
            if let Err(e) = bincode::serialize_into(
                &player.stream, 
                &FromServer::YourTurn
            ) {
                println!("Leave lobby message failed to send {e}");
                // TODO: might need to disconnect client here, or return to lobby?
            }
        }
    }

}