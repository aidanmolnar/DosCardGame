use super::lobby_network::*;
use super::graphics::*;
use dos_shared::*;
use dos_shared::cards::*;

use bevy::prelude::*;

//use iyes_loopless::prelude::*;

use std::net::TcpStream;
use std::io;


// Recieves and handles messages from the server
pub fn game_network_system(
    mut mp_state: ResMut<MultiplayerState>, 
    commands: Commands,
) {
    let stream =
        match &mp_state.stream {
            None => return,
            Some(i) => i,
    };
    
    match bincode::deserialize_from::<&TcpStream, GameUpdateServer>(stream) {
        Ok(game_update) => {
            handle_game_update(
            game_update,
            commands,
            mp_state)
        },
        Err(e) => {
            handle_game_update_error(&mut mp_state, e)
        }
    }
}

fn handle_game_update(
    game_update: GameUpdateServer, 
    commands: Commands,
    mp_state: ResMut<MultiplayerState>) {
    match game_update {
        GameUpdateServer::DealIn { your_cards, card_counts } => {
            println!("Got cards: {:?}", your_cards);
            deal_out_cards(
                your_cards, 
                card_counts,
                commands,
                mp_state,
            );
        }
    }
}



// Move to grpahics?
fn deal_out_cards(
    your_cards: Vec<Card>, 
    mut card_counts: Vec<u8>,
    mut commands: Commands,
    mp_state: ResMut<MultiplayerState>,
) {

    let delay_delta = 0.25;
    let mut delay_total = 0.0;

    // Deal out the hands from the deck
    // This is probably more complicated than it needs to be, can make assumptions about how server deals out cards.  Remove card counts from message?
    // TODO: Simplify
    for j in 0..NUM_STARTING_CARDS {
        for (card_owner_id,count) in card_counts.iter_mut().enumerate() {
            if *count > 0 {
                *count -= 1;

                let card_value = if card_owner_id == mp_state.turn_id as usize {
                    Some(*your_cards.get(j as usize).unwrap())
                } else {
                    None
                };

                commands.spawn().insert(DelayedDealtCard {
                    timer: Timer::from_seconds(delay_total, false),
                    owner_id: card_owner_id as u8,
                    card_value,
                });

                delay_total += delay_delta;
                
            } else {
                break;
            }
        }
    }
}

// Checks if error is just non-blocking error
// Otherwise disconnects
fn handle_game_update_error(mp_state: &mut ResMut<MultiplayerState>, e: Box<bincode::ErrorKind>) {
    match *e {
        bincode::ErrorKind::Io(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
        _ => {
            println!("Message receive error: {}", e);
            println!("Disconnecting!");

            disconnect(mp_state);

            // TODO: return to lobby?
        }
    }
}