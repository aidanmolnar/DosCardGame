use dos_shared::dos_game::DosGame;
use dos_shared::messages::game::*;

use crate::connection_listening::{PlayerCountChange, disconnect};
use super::server_game::ServerGame;
use super::multiplayer::{NetPlayer, Agent, AgentTracker};

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use::bincode;
use std::net::TcpStream;
use std::io;
use std::io::Write;

#[derive(SystemParam)]
pub struct GameNetworkManager<'w,'s> {
    events: EventWriter<'w, 's, PlayerCountChange>, 
    commands: Commands<'w, 's>,
    agent_tracker: ResMut<'w, AgentTracker>,
    pub card_tracker: ServerGame<'w,'s>,
}

// TODO: Incorporate query so it doesn't need to be passed around
pub fn game_network_system (
    query: Query<(Entity, &NetPlayer, &Agent)>, 
    mut manager: GameNetworkManager
) {
    for (entity, player, agent) in query.iter() {
        match bincode::deserialize_from::<&TcpStream, FromClient>(&player.stream) {
            Ok(update) => {
                manager.handle_update(
                    &query,
                    update,   
                    agent, 
                );
            },
            Err(e) => {
                manager.handle_error(
                    e, 
                    entity, 
                    player, 
                );
            }
        }
    }
}

impl<'w,'s> GameNetworkManager<'w,'s> {

    // TODO: Replace panics
    fn handle_update(
        &mut self,
        query: &Query<(Entity, &NetPlayer, &Agent)>, 
        update: FromClient, 
        agent: &Agent, 
    ) {
        let action;

        match update.0 {
            GameAction::PlayCard (card)=> {
                if self.card_tracker.validate_play_card(agent.turn_id, &card) {
                    self.card_tracker.play_card(&card);

                    action = Some(GameAction::PlayCard(card));
                } else {
                    panic!("Invalid play card");
                }
            },
            GameAction::DrawCards => {
                if self.card_tracker.validate_draw_cards(agent.turn_id) {
                    self.card_tracker.draw_cards();

                    // TODO: Clean this up.  Issue is that all clients need to receive this message.  Including sender.
                    let conditions = self.card_tracker.syncer.take_conditions();
                    for (_, _, agent) in query.iter() {
                        let cards = self.card_tracker.syncer.take_player_cards(agent.turn_id);
                        self.send_to_one(query, 
                            FromServer {
                                action: GameAction::DrawCards, 
                                conditions: conditions.clone(), 
                                cards
                            }, 
                            agent.turn_id
                        )
                    }
                    return;
                } else {
                    panic!("Invalid draw cards");
                }
            },
            GameAction::KeepStaging => {
                if self.card_tracker.validate_keep_last_drawn_card(agent.turn_id) {
                    self.card_tracker.keep_last_drawn_card();

                    action = Some(GameAction::KeepStaging);
                } else {
                    panic!("Invalid keep last drawn card");
                }
            },
            GameAction::DiscardWildColor(color) => {
                if self.card_tracker.validate_declare_wildcard_color(agent.turn_id, &color) {
                    self.card_tracker.declare_wildcard_color(&color);

                    action = Some(GameAction::DiscardWildColor(color));
                } else {
                    panic!("Invalid wildcard select color");
                }
            },
            _ => {
                panic!("Invalid client action")
            }
        }

        if let Some(action) = action {
            let conditions = self.card_tracker.syncer.take_conditions();
            
            for (_, _, a) in query.iter().filter(|x|x.2.turn_id != agent.turn_id) {
                let cards = self.card_tracker.syncer.take_player_cards(a.turn_id);
                self.send_to_one(query, 
                    FromServer{
                        action, 
                        conditions: conditions.clone(), 
                        cards
                    }, 
                    a.turn_id
                )
            }
        }
    }

    pub fn send_to_one(
        &mut self,
        query: &Query<(Entity, &NetPlayer, &Agent)>, 
        message: FromServer,
        receiver_id: usize,
    ) {
        let bytes = bincode::serialize(&message).expect("Failed to serialize message");
        
        for (_, player, agent) in query.iter() {
            if agent.turn_id == receiver_id {
                // TODO: Cloning the stream is not the best way to handle having an immutable reference here
                if let Err(e) =  player.stream.try_clone().unwrap().write_all(&bytes) {
                    panic!("Leave lobby message failed to send {e}");
                    // TODO: might need to disconnect client here, or return to lobby?
                }
                return;
            }
        }
    }

    // Checks if error is just non-blocking error
    fn handle_error(
        &mut self,
        e: Box<bincode::ErrorKind>,
        entity: Entity,
        player: &NetPlayer,
    ) {
        match *e {
            bincode::ErrorKind::Io(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            _ => {
                println!("Message receive error: {}", e);

                // TODO: Should disconnect be a method... probably
                disconnect(
                    entity, 
                    player, 
                    &mut self.events, 
                    &mut self.commands, 
                    &mut self.agent_tracker
                );
            }
        }
    }
}







