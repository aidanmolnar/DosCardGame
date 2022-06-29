use dos_shared::cards::*;
use dos_shared::*;
use super::lobby_network::*;

use bevy::prelude::*;


const NUM_STARTING_CARDS: u8 = 10;

#[derive(Debug)]
pub struct GameState {
    deck: Vec<Card>,
    discard_pile: Vec<Card>,
    current_turn: u8,
}

#[derive(Component, Clone)]
pub struct Hand {
    cards: Vec<Card>
}

pub fn enter_game_system(
    mut commands: Commands,
    query: Query<(Entity, &NetPlayer)>,
) {
    let mut deck = new_deck(); // Get a standard shuffled deck of cards

    // Create a vector of "hands" of cards
    let mut hands = Vec::new();
    for _ in 0..query.iter().len() {
        hands.push(Hand {cards: Vec::new()});
    }

    // Deal out the hands from the deck
    for _ in 0..NUM_STARTING_CARDS {
        for hand in hands.iter_mut() {
            if let Some(card) = deck.pop() {
                hand.cards.push(card);
            } else {
                break; // Deck is out of cards
            }
        }
    }
    
    // TODO: there is probably a better/more functional way to do this that doesn't require cloning the hands
    for (i,(entity, player)) in query.iter().enumerate() {
        let hand = hands.get(i).unwrap();
        commands.entity(entity).insert(hand.clone());

        if let Err(e) = bincode::serialize_into(&player.stream, &GameUpdateServer::DealIn{cards: hand.cards.clone()}) {
            println!("Leave lobby message failed to send {e}");
            // TODO: might need to disconnect client here, or return to lobby?
        }
    }

    commands.insert_resource(
        GameState {
            deck,
            discard_pile: Vec::new(),
            current_turn: 0,
        }
    );
}