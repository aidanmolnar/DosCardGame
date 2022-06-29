
use super::lobby_network::*;
use dos_shared::*;
use dos_shared::cards::*;

use bevy::prelude::*;
//use iyes_loopless::prelude::*;

use std::net::TcpStream;
use std::io;

struct GameState {
    hand: Vec<Card>, // Maybe each card should be its own entity?
}

// Recieves and handles messages from the server
pub fn game_network_system(
    mut mp_state: ResMut<MultiplayerState>, 
    mut commands: Commands
) {
    let stream =
        match &mp_state.stream {
            None => return,
            Some(i) => i,
    };
    
    match bincode::deserialize_from::<&TcpStream, GameUpdateServer>(stream) {
        Ok(game_update) => {
            match game_update {
                GameUpdateServer::DealIn { cards } => {
                    println!("{:?}", cards);
                    
                    commands.insert_resource( GameState {
                        hand: cards,
                    })
                }
            }
        },
        Err(e) => {
            handle_game_update_error(&mut mp_state, e)
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