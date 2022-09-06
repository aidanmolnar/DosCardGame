use dos_shared::dos_game::DosGame;
use dos_shared::messages::game::*;

use super::server_game::ServerGame;
use super::call_dos::CallDos;
use super::multiplayer::AgentTracker;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use::bincode;

use std::net::TcpStream;
use std::io;
use std::io::Write;



#[derive(SystemParam)]
pub struct GameNetworkManager<'w,'s> {
    pub game: ServerGame<'w,'s>,
    
    agent_tracker: ResMut<'w, AgentTracker>,
}

pub fn game_network_system (
    mut manager: GameNetworkManager,
    
) {
    // Loop over all the streams
    for player in 0..manager.agent_tracker.num_agents() {
        if let Some(stream) = manager.agent_tracker.try_get_stream(player) {

            // Handle message from this stream
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
                    println!("Invalid play card");
                    self.handle_disconnect(player);
                }
            },
            GameAction::DrawCards => {
                if self.game.validate_draw_cards(player) {
                    self.game.draw_cards();

                    self.send_to_all(GameAction::DrawCards)
                } else {
                    println!("Invalid draw cards");
                    self.handle_disconnect(player);
                }
            },
            GameAction::KeepStaging => {
                if self.game.validate_keep_last_drawn_card(player) {
                    self.game.keep_last_drawn_card();

                    self.send_to_filtered(GameAction::KeepStaging, |p|p!=player)
                } else {
                    println!("Invalid keep last drawn card");
                    self.handle_disconnect(player);
                }
            },
            GameAction::DiscardWildColor(color) => {
                if self.game.validate_declare_wildcard_color(player, &color) {
                    self.game.declare_wildcard_color(&color);

                    self.send_to_filtered(GameAction::DiscardWildColor(color), |p|p!=player)
                } else {
                    println!("Invalid wildcard select color");
                    self.handle_disconnect(player);
                }
            },
            GameAction::CallDos{..} => {
                if let Some(call_dos) = &mut self.game.call_dos {
                    if player == call_dos.player {
                        let action = GameAction::CallDos (
                            Some(CallDosInfo {
                                player: call_dos.player,
                                caller: call_dos.player,
                            })
                        );
                        // Remove call dos, send message that someone called it
                        self.game.commands.remove_resource::<CallDos>();
                        self.send_to_all(action);
                    } else {
                        // Start the timer running if it is not already running
                        if call_dos.graceperiod.is_none() {
                            call_dos.caller = Some(player);
                            call_dos.graceperiod = Some(Timer::from_seconds(0.5, false));
                        }
                    }
                } else {
                    // This isn't necessarily a desync, just client not receiving call dos message yet.
                    println!("Invalid call dos");
                }
            }
            _ => {
                println!("Invalid client action");
                self.handle_disconnect(player);
            }
        }
    }

    
    pub fn send_to_filtered<F> (
        &mut self,
        action: GameAction,
        filter: F // Takes player_id as argument. Sends message if true.
    ) where F: Fn(usize) -> bool{
        let conditions = self.game.syncer.take_conditions();

        // Keep track of players that have errors on send
        let mut disconnects = Vec::new();

        // Loop over playes that meet condition
        for (player, mut stream) in self.agent_tracker.iter_ids_and_streams()
        .filter(
            |(player,_)| filter(*player)
        ) {
            let cards = self.game.syncer.take_player_cards(player);
            let bytes = bincode::serialize(&FromServer{
                action: action.clone(), 
                conditions: conditions.clone(), 
                cards
            }).expect("Failed to serialize message");

            if let Err(e) =  stream.write_all(&bytes) {
                println!("Message failed to send {e}");
                disconnects.push(player);
            }
        }

        // Deal with errored players
        for player in disconnects {
            self.handle_disconnect(player);
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
                println!("Message failed to receive: {}", e);
                self.handle_disconnect(player);
            }
        }
    }

    fn handle_disconnect(
        &mut self,
        player: usize,
    ) {
        println!("Player disconnected: {}", player);
        
        self.agent_tracker.disconnect_player(player);
    }
}







