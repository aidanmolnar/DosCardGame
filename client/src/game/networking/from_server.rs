use dos_shared::GameInfo;
use dos_shared::messages::game::*;
use crate::game::server_actions::dealing::create_delayed_transfers;
use crate::game::table::CardTransferer;

use super::MultiplayerState;

use super::server_actions::deal_out_cards;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use std::net::TcpStream;
use std::io;
use std::io::Write;


#[derive(SystemParam)]
pub struct GameNetworkManager<'w, 's> {
    pub mp_state: ResMut<'w, MultiplayerState>,
    pub commands: Commands<'w,'s>,
    pub card_transferer: CardTransferer<'w,'s>,
    pub game_info: ResMut<'w, GameInfo>,
}

// Recieves and handles messages from the server
pub fn game_network_system(
    mut network_manager: GameNetworkManager,
) {
    let stream =
        match &network_manager.mp_state.stream {
            None => return,
            Some(i) => i,
    };
    
    match bincode::deserialize_from::<&TcpStream, FromServer>(stream) {
        Ok(game_update) => {
            network_manager.handle_update(
                game_update
            )
        },
        Err(e) => {
            network_manager.handle_error(e)
        }
    }
}

impl<'w,'s> GameNetworkManager<'w,'s> {
    fn handle_update(&mut self, game_update: FromServer) {
        match game_update {
            FromServer::DealIn { your_cards, deck_size, to_discard_pile} => {
                println!("Got cards: {:?}", your_cards);
                println!("Deck size: {:?}", deck_size);
    
                deal_out_cards (
                    your_cards, 
                    deck_size,
                    to_discard_pile,
                    &mut self.commands,
                    &mut self.mp_state,
                );
            }
            FromServer::NextTurn => {
                println!("Next turn!");
                self.game_info.next_turn();
            }
            FromServer::TransferCards(transfers) => {
                create_delayed_transfers(&mut self.commands, transfers, 0.5);
            }
        }
    }

    fn handle_error(&mut self, e: Box<bincode::ErrorKind>) {
        match *e {
            bincode::ErrorKind::Io(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            _ => {
                println!("Message receive error: {}", e);
                println!("Disconnecting!");
    
                self.mp_state.set_disconnected();
                // TODO: return to lobby?
            }
        }
    }

    pub fn send_message(&mut self, message: FromClient) {
        self.mp_state.stream.as_ref().unwrap()
        .write_all(
            &bincode::serialize(&message).unwrap()
        ).expect("Failed to send message");
        // NOTE: Using bincode::serialize_into was causing crashes related to enum discriminants
        //       Not completely sure why it seems to work elsewhere.  
        //      bincode::serialize and bincode::serialize_into do have different default behavior and this seems to solve the issue here
    }

    pub fn is_your_turn(&self) -> bool {
        self.game_info.current_turn() == self.mp_state.turn_id
    }
}