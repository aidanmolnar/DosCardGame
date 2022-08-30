use dos_shared::dos_game::DosGame;
use dos_shared::messages::game::*;

use crate::connection_listening::{PlayerCountChange, disconnect};
use super::server_game::ServerGame;
use super::multiplayer::AgentTracker;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use::bincode;

use std::net::TcpStream;
use std::io;
use std::io::Write;



#[derive(SystemParam)]
pub struct GameNetworkManager<'w,'s> {
    events: EventWriter<'w, 's, PlayerCountChange>, 
    pub game: ServerGame<'w,'s>,

    agent_tracker: Res<'w, AgentTracker>,
}

pub fn game_network_system (
    mut manager: GameNetworkManager,
    
) {
    for player in 0..manager.agent_tracker.num_agents() {
        if let Some(stream) = manager.agent_tracker.try_get_stream(player) {

            match bincode::deserialize_from::<&TcpStream, FromClient>(stream) {
                Ok(update) => {
                    manager.handle_update(
                        update,   
                        player, 
                    );
                },
                Err(e) => {
                    manager.handle_error(
                        e, 
                        player, 
                    );
                }
            }

        }
    }
}

impl<'w,'s> GameNetworkManager<'w,'s> {

    // TODO: Replace panics
    fn handle_update(
        &mut self,
        update: FromClient, 
        player: usize,
    ) {
        match update.0 {
            GameAction::PlayCard (card)=> {
                if self.game.validate_play_card(player, &card) {
                    self.game.play_card(&card);

                    self.send_to_filtered(GameAction::PlayCard(card), |p|p!=player)
                } else {
                    panic!("Invalid play card");
                }
            },
            GameAction::DrawCards => {
                if self.game.validate_draw_cards(player) {
                    self.game.draw_cards();

                    self.send_to_all(GameAction::DrawCards)
                } else {
                    panic!("Invalid draw cards");
                }
            },
            GameAction::KeepStaging => {
                if self.game.validate_keep_last_drawn_card(player) {
                    self.game.keep_last_drawn_card();

                    self.send_to_filtered(GameAction::KeepStaging, |p|p!=player)
                } else {
                    panic!("Invalid keep last drawn card");
                }
            },
            GameAction::DiscardWildColor(color) => {
                if self.game.validate_declare_wildcard_color(player, &color) {
                    self.game.declare_wildcard_color(&color);

                    self.send_to_filtered(GameAction::DiscardWildColor(color), |p|p!=player)
                } else {
                    panic!("Invalid wildcard select color");
                }
            },
            _ => {
                panic!("Invalid client action")
            }
        }
    }

    pub fn send_to_filtered<F> (
        &mut self,
        action: GameAction,
        filter: F
    ) where F: Fn(usize) -> bool{
        let conditions = self.game.syncer.take_conditions();

        for (player, mut stream) in self.agent_tracker.iter_ids_and_streams()
        .filter(|(player,_)|filter(*player)) {

            let cards = self.game.syncer.take_player_cards(player);
            let bytes = bincode::serialize(&FromServer{
                action, 
                conditions: conditions.clone(), 
                cards
            }).expect("Failed to serialize message");

            if let Err(e) =  stream.write_all(&bytes) {
                panic!("Leave lobby message failed to send {e}");
                // TODO: might need to disconnect client here, or return to lobby?
            }
        }
    }   

    pub fn send_to_all(
        &mut self,
        action: GameAction,
    ) {
        self.send_to_filtered(action, |_|{true})
    }

    // Checks if error is just non-blocking error
    fn handle_error(
        &mut self,
        e: Box<bincode::ErrorKind>,
        player: usize,
    ) {
        match *e {
            bincode::ErrorKind::Io(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            _ => {
                println!("Message receive error: {}", e);

                disconnect(
                    player, 
                    &mut self.events, 
                );
            }
        }
    }
}







