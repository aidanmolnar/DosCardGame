use dos_shared::*;
use super::MultiplayerState;

use super::dealing::deal_out_cards;

use bevy::prelude::*;

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
        GameUpdateServer::YourTurn => {
            println!("Your turn!");
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

            mp_state.set_disconnected();

            // TODO: return to lobby?
        }
    }
}