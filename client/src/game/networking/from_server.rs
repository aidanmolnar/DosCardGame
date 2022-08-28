use dos_shared::dos_game::DosGame;
use dos_shared::messages::game::*;
use dos_shared::DECK_SIZE;
    
use crate::game::table::ClientCardTracker;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use std::net::TcpStream;
use std::io;
use std::io::Write;


#[derive(SystemParam)]
pub struct GameNetworkManager<'w, 's> {
    pub commands: Commands<'w,'s>,
    pub card_tracker: ClientCardTracker<'w,'s>,
}

// Recieves and handles messages from the server
pub fn game_network_system(
    mut network_manager: GameNetworkManager,
) {
    let stream =
        match &network_manager.card_tracker.mp_state.stream {
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

        self.card_tracker.memorized_cards.1 = game_update.condition_counter;

        for card in game_update.cards {
            self.card_tracker.memorized_cards.enque(card);
        }

        match game_update.action {
            GameAction::DealIn => {
                println!("{:?}", self.card_tracker.memorized_cards);

                self.card_tracker.deal_starting_cards(DECK_SIZE)
            },
            GameAction::PlayCard(card) => {
                self.card_tracker.play_card(&card);
            },
            GameAction::DrawCards => {
                self.card_tracker.draw_cards();
            },
            GameAction::KeepStaging => {
                self.card_tracker.keep_last_drawn_card();
            },
            GameAction::DiscardWildColor(color) => {
                self.card_tracker.declare_wildcard_color(&color);
            },
        }
    }

    fn handle_error(&mut self, e: Box<bincode::ErrorKind>) {
        match *e {
            bincode::ErrorKind::Io(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            _ => {
                println!("Message receive error: {}", e);
                println!("Disconnecting!");
    
                self.card_tracker.mp_state.set_disconnected();
                // TODO: return to lobby?
            }
        }
    }

    pub fn send_message(&mut self, message: FromClient) {
        self.card_tracker.mp_state.stream.as_ref().unwrap()
        .write_all(
            &bincode::serialize(&message).unwrap()
        ).expect("Failed to send message");
        // NOTE: Using bincode::serialize_into was causing crashes related to enum discriminants
        //       Not completely sure why it seems to work elsewhere.  
        //      bincode::serialize and bincode::serialize_into do have different default behavior and this seems to solve the issue here
    }
}