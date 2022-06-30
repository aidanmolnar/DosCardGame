
use super::lobby_network::*;
use super::graphics::*;
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
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    card_atlas: Res<CardTetxureAtlas>,
) {
    let stream =
        match &mp_state.stream {
            None => return,
            Some(i) => i,
    };
    
    match bincode::deserialize_from::<&TcpStream, GameUpdateServer>(stream) {
        Ok(game_update) => {
            match game_update {
                GameUpdateServer::DealIn { mut cards } => {
                    println!("Original: {:?}", cards);

                    cards.sort();

                    println!("Sorted: {:?}", cards);
                    
                    commands.insert_resource( GameState {
                        hand: cards.clone(),
                    });

                    
                    for card in cards.iter() {
                        add_card(card, Vec3::new(0.,0.,0.), &mut commands, &card_atlas, &texture_atlases);
                    }

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