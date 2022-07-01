
use super::lobby_network::*;
use super::graphics::*;
use dos_shared::*;
//use dos_shared::cards::*;

use bevy::prelude::*;
//use iyes_loopless::prelude::*;

use std::net::TcpStream;
use std::io;



// struct GameState {
//     hand: Vec<Card>, // Maybe each card should be its own entity?
// }

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
                GameUpdateServer::DealIn { your_cards: cards, mut card_counts } => {
                    println!("Got cards: {:?}", cards);

                    // Deal out the hands from the deck
                    for j in 0..NUM_STARTING_CARDS {
                        for (i,count) in card_counts.iter_mut().enumerate() {
                            if *count > 0 {
                                *count -= 1;

                                if i as u8 == mp_state.turn_id {
                                    add_your_card(
                                        *cards.get(j as usize).unwrap(), 
                                        Vec3::new(0.,0.,0.), 
                                        &mut commands, &card_atlas, 
                                        &texture_atlases
                                    );
                                } else {
                                    add_other_card(
                                        i as u8, 
                                        j as u8,
                                        Vec3::new(0.,0.,0.), 
                                        &mut commands, &card_atlas, 
                                        &texture_atlases
                                    );
                                }


                            } else {
                                break;
                            }
                        }
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