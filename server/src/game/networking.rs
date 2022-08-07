use dos_shared::messages::game::*;
use super::multiplayer::{NetPlayer, Agent, AgentTracker};
use super::super::connection_listening::{PlayerCountChange, disconnect};

use bevy::prelude::*;

use::bincode;
use std::net::TcpStream;
use std::io;


pub fn game_network_system (
    query: Query<(Entity, &NetPlayer, &Agent)>, 
    mut events: EventWriter<PlayerCountChange>, 
    mut commands: Commands,
    mut agent_tracker: ResMut<AgentTracker>,
) {
    for (entity, player, agent) in query.iter() {
        match bincode::deserialize_from::<&TcpStream, FromClient>(&player.stream) {
            Ok(lobby_update) => {
                handle_game_update(
                    lobby_update, 
                    agent, 
                    &mut commands, 
                );
            },
            Err(e) => {
                handle_game_update_error(
                    e, 
                    entity, 
                    player, 
                    &mut events, 
                    &mut commands,
                    &mut agent_tracker,
                );
            }
        }
    }
}

fn handle_game_update(
    lobby_update: FromClient, 
    agent: &Agent, 
    commands: &mut Commands
) {
    match lobby_update {
        FromClient::PlayCard{card} => {
            println!("Client played a card {:?}", card);
        }
        
    }
}

// Checks if error is just non-blocking error
fn handle_game_update_error(
        e: Box<bincode::ErrorKind>,
        entity: Entity,
        player: &NetPlayer,
        events: &mut EventWriter<PlayerCountChange>, 
        commands: &mut Commands,
        agent_tracker: &mut ResMut<AgentTracker>,
) {
    match *e {
        bincode::ErrorKind::Io(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
        _ => {
            println!("Message receive error: {}", e);

            disconnect(entity, player, events, commands, agent_tracker);
        }
    }
}




