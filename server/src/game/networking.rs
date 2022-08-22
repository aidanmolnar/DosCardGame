use dos_shared::cards::CardColor;
use dos_shared::table::*;
use dos_shared::valid_move;
use dos_shared::messages::game::*;
use dos_shared::GameInfo;

use super::multiplayer::{NetPlayer, Agent, AgentTracker};
use super::super::connection_listening::{PlayerCountChange, disconnect};
use super::table::CardTransferer;

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
    card_transferer: CardTransferer<'w,'s>,
    game_info: ResMut<'w, GameInfo>,
    can_play_wild: Local<'s, bool>,
}

// TODO: Incorporate query so it doesn't need to be passed around
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
        if agent.turn_id != self.game_info.current_turn() {
            // TODO: Disconnect the player or something
            panic!("Someone played out of turn");
        }

        match lobby_update {
            FromClient::PlayCard{card: card_reference} => {
                self.handle_play_card(
                    query,
                    agent,
                    card_reference
                )
            }
            FromClient::DrawCards => {
                // Store references for card transfers
                let from = CardReference{
                    location: Location::Deck,
                    index: None
                };
                let to = CardReference{
                    location: Location::Hand{player_id: agent.turn_id},
                    index: None
                };
                let to_last = CardReference{
                    location: Location::Staging,
                    index: None
                };

                let discard_pile = self.card_transferer.peek_discard().unwrap();
                let mut agent_transfers = Vec::new();
                let mut other_transfers = Vec::new();

                loop {
                    // We can be sure the deck will have cards because card transferer will automatically reshuffle pile
                    let card = self.card_transferer.peek(&from).unwrap();

                    if valid_move(card, discard_pile) {
                        // Others think card was transfered to players hand
                        other_transfers.push(CardTransfer {from, to, value: None});
                        agent_transfers.push(CardTransfer {from, to: to_last, value: Some(card)});
                        self.card_transferer.transfer(from, to_last);
                        break
                    } else {
                        other_transfers.push(CardTransfer {from, to, value: None});
                        agent_transfers.push(CardTransfer {from, to, value: Some(card)});
                        self.card_transferer.transfer(from, to);
                    }
                }

                // Update the player who drew with the transfers and card values
                self.send_to_one(
                    query, 
                    FromServer::TransferCards(agent_transfers), 
                    agent.turn_id
                );

                // Update the other players with just the transfers
                self.broadcast(
                    query, 
                    FromServer::TransferCards(other_transfers), 
                    agent.turn_id
                );

                // Do not update turn, player has to decide whether to play staged card or not...
            }
            FromClient::KeepStaging => {
                if self.card_transferer.peek_staging().is_some() {
                    self.card_transferer.transfer(
                        CardReference { location: Location::Staging, index: None },
                        CardReference{ location: Location::Hand{player_id: agent.turn_id}, index: None}
                    );

                    self.broadcast(
                        query, 
                        FromServer::NextTurn,
                    agent.turn_id
                    );
                    self.game_info.next_turn();
                } else {
                    //TODO: Disconnect client/ treat as desync?
                    // Call handle error?
                    panic!("Client desynced")
                }
            }
            FromClient::DiscardWildColor(color) => {
                if *self.can_play_wild {
                    *self.can_play_wild = false;

                    let mut card = self.card_transferer.peek_discard().unwrap();
                    card.color = color;

                    // Update the card color in the table
                    self.card_transferer.set_discard_value(card);

                    // Send a message to the clients
                    self.broadcast(
                        query, 
                        FromServer::DiscardWildColor(color),
                    agent.turn_id
                    );

                    self.broadcast(
                        query, 
                        FromServer::NextTurn,
                    agent.turn_id
                    );
                    self.game_info.next_turn();

                } else {
                    // TODO: Should not panic
                    panic!("Client desynced, shouldn't be playing wild card")
                }
            }
            
        }
    }

    fn handle_play_card(
        &mut self,
        query: &Query<(Entity, &NetPlayer, &Agent)>,
        agent: &Agent,
        card_reference: CardReference,
    ) {
        println!("Client played a card {:?}", card_reference);

        if card_reference.location != (Location::Hand{player_id: agent.turn_id}) && 
        card_reference.location != (Location::Staging){
            // TODO: Hand disconnect / desync
            panic!("Player attempted to play card they could not have control over")
        }
        
        let discard_pile = self.card_transferer.peek_discard().unwrap();

        // TODO: Clean this up
        if let Some(card) = self.card_transferer.peek(&card_reference) {
            if valid_move(card, discard_pile) && self.game_info.current_turn() == agent.turn_id {
                self.card_transferer.transfer(
                    card_reference, 
                    CardReference{
                        location: Location::DiscardPile,
                        index: None
                    }
                );

                let mut from = card_reference;

                if from.location == Location::Staging {
                    from.location = Location::Hand{player_id: agent.turn_id};
                    from.index = None;
                }
    
                let transfer = CardTransfer {
                    from,
                    to: CardReference{
                        location: Location::DiscardPile,
                        index: None
                    },
                    value: Some(card),
                };
                
                self.broadcast(
                    query, 
                    FromServer::TransferCards (vec![transfer]),
                agent.turn_id
                );
    
                
                if card.color == CardColor::Wild {
                    *self.can_play_wild = true;
                } else {
                    // TODO: Shouldn't have to iterate over all agents/players again...
                    self.broadcast(
                        query, 
                        FromServer::NextTurn,
                    agent.turn_id
                    );
                    self.game_info.next_turn();
                }
    
            } else {
                //TODO: Disconnect client/ treat as desync?
                // Call handle error?
                panic!("Client desynced")
            }
        } else {
            //TODO: Disconnect client/ treat as desync?
                // Call handle error?
                panic!("Client desynced")
        }

        

    }


    fn broadcast(
        &mut self,
        query: &Query<(Entity, &NetPlayer, &Agent)>, 
        message: FromServer,
        skip_player_id: usize,
    ) {
        let bytes = bincode::serialize(&message).expect("Failed to serialize message");

        for (_, player, agent) in query.iter() {
            if agent.turn_id != skip_player_id {
                // TODO: Cloning the stream is not the best way to handle having an immutable reference here
                if let Err(e) =  player.stream.try_clone().unwrap().write_all(&bytes) {
                    panic!("Leave lobby message failed to send {e}");
                    // TODO: might need to disconnect client here, or return to lobby?
                }
            }
        }
    }

    fn send_to_one(
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







