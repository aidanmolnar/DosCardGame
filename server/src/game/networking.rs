use dos_shared::table::*;
use dos_shared::valid_move;
use dos_shared::messages::game::*;
use super::multiplayer::{NetPlayer, Agent, AgentTracker};
use super::super::connection_listening::{PlayerCountChange, disconnect};
use super::table::CardTransferer;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use::bincode;
use std::net::TcpStream;
use std::io;

#[derive(SystemParam)]
pub struct GameNetworkManager<'w,'s> {
    events: EventWriter<'w, 's, PlayerCountChange>, 
    commands: Commands<'w, 's>,
    agent_tracker: ResMut<'w, AgentTracker>,
    card_transferer: CardTransferer<'w,'s>,
}

pub fn game_network_system (
    query: Query<(Entity, &NetPlayer, &Agent)>, 
    mut manager: GameNetworkManager
) {
    for (entity, player, agent) in query.iter() {
        match bincode::deserialize_from::<&TcpStream, FromClient>(&player.stream) {
            Ok(lobby_update) => {
                manager.handle_update(
                    &query,
                    lobby_update,   
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
    fn handle_update(
        &mut self,
        query: &Query<(Entity, &NetPlayer, &Agent)>, 
        lobby_update: FromClient, 
        agent: &Agent, 
    ) {
        match lobby_update {
            FromClient::PlayCard{card: card_reference} => {
                println!("Client played a card {:?}", card_reference);

                if card_reference.location != (Location::Hand{player_id: agent.turn_id}) {return}
                let card = self.card_transferer.peek(&card_reference);
                let discard_pile = self.card_transferer.peek_discard().unwrap();

                if valid_move(card, discard_pile) {
                    self.card_transferer.transfer(
                        card_reference, 
                        CardReference{
                            location: Location::DiscardPile,
                            index: None
                        }
                    );

                    self.broadcast(
                        query, 
                        FromServer::TransferCard { 
                            from: card_reference,
                            to: CardReference{
                                location: Location::DiscardPile,
                                index: None
                            },
                            value: Some(card),
                        },
                    agent.turn_id
                    );

                } else {
                    //TODO: Disconnect client/ treat as desync?
                    // Call handle error?
                    panic!("Client desynced")
                }

            }
            
        }
    }

    fn broadcast(
        &mut self,
        query: &Query<(Entity, &NetPlayer, &Agent)>, 
        message: FromServer,
        skip_player_id: usize,
    ) {
        for (_, player, agent) in query.iter() {
            if agent.turn_id != skip_player_id {
                
                if let Err(e) = bincode::serialize_into(&player.stream, &message) {
                    panic!("Leave lobby message failed to send {e}");
                    // TODO: might need to disconnect client here, or return to lobby?
                }
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







