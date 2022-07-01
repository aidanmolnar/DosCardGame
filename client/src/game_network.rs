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
    texture_atlases: Res<Assets<TextureAtlas>>,
    card_atlas: Res<CardTetxureAtlas>,
    card_tracker: ResMut<CardTracker>,
    events: EventWriter<CardChanged>,
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
                &mut ResourceReference {
                    commands,
                    mp_state,
                    card_tracker,
                    events,
                    texture_atlases,
                    card_atlas,
                })
        },
        Err(e) => {
            handle_game_update_error(&mut mp_state, e)
        }
    }
}

fn handle_game_update(game_update: GameUpdateServer, reference: &mut ResourceReference) {
    match game_update {
        GameUpdateServer::DealIn { your_cards, card_counts } => {
            println!("Got cards: {:?}", your_cards);
            deal_cards(
                your_cards, 
                card_counts,
                reference,
            );
        }
    }
}

// Bundle of resources to make passing information to functions cleaner
// TODO: rename/reconsider
pub struct ResourceReference<'a,'b> {
    pub commands: Commands<'a, 'b>,
    pub mp_state: ResMut<'b, MultiplayerState>, 
    pub card_tracker: ResMut<'b, CardTracker>,
    pub events: EventWriter<'b,'b,  CardChanged>,
    pub texture_atlases: Res<'b, Assets<TextureAtlas>>,
    pub card_atlas: Res<'b, CardTetxureAtlas>,
}

fn deal_cards(
    your_cards: Vec<Card>, 
    mut card_counts: Vec<u8>,
    reference: &mut ResourceReference
) {
    // Deal out the hands from the deck
    for j in 0..NUM_STARTING_CARDS {
        for (i,count) in card_counts.iter_mut().enumerate() {
            if *count > 0 {
                *count -= 1;

                let card_value = if i == reference.mp_state.turn_id as usize {
                    Some(*your_cards.get(j as usize).unwrap())
                } else {
                    None
                };

                deal_card(
                    i as u8,
                    card_value, 
                    reference);
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