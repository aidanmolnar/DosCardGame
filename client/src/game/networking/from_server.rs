use dos_shared::messages::game::*;
use crate::game::table::CardTransferer;

use super::MultiplayerState;

use super::server_actions::deal_out_cards;

use bevy::prelude::*;

use std::net::TcpStream;
use std::io;

#[derive(Default)]
pub struct YourTurn;

// Recieves and handles messages from the server
pub fn game_network_system(
    mut mp_state: ResMut<MultiplayerState>, 
    commands: Commands,
    mut card_transferer: CardTransferer,
) {
    let stream =
        match &mp_state.stream {
            None => return,
            Some(i) => i,
    };
    
    match bincode::deserialize_from::<&TcpStream,FromServer>(stream) {
        Ok(game_update) => {
            handle_game_update(
                game_update,
                commands,
                mp_state,
                &mut card_transferer
            )
        },
        Err(e) => {
            handle_game_update_error(&mut mp_state, e)
        }
    }
}

fn handle_game_update(
    game_update: FromServer, 
    mut commands: Commands,
    mp_state: ResMut<MultiplayerState>,
    card_transferer: &mut CardTransferer,
) {
    match game_update {
        FromServer::DealIn { your_cards, deck_size, to_discard_pile} => {
            println!("Got cards: {:?}", your_cards);
            println!("Deck size: {:?}", deck_size);

            deal_out_cards(
                your_cards, 
                deck_size,
                to_discard_pile,
                commands,
                mp_state,
            );
        }
        FromServer::YourTurn => {
            println!("Your turn!");
            commands.init_resource::<YourTurn>();
        }
        FromServer::TransferCard{from, to, value} => {
            card_transferer.transfer(from, to, value);
        }
    }
}

// Checks if error is just non-blocking error
// Otherwise disconnects
fn handle_game_update_error(
    mp_state: &mut ResMut<MultiplayerState>, 
    e: Box<bincode::ErrorKind>
) {
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