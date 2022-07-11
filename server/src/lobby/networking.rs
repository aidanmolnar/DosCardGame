use dos_shared::messages::lobby::*;
use super::GameState;
use super::multiplayer::{NetPlayer, Agent, AgentTracker};
use super::connection_listening::{PlayerCountChange, disconnect};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use::bincode;
use std::net::TcpStream;
use std::io;

// Runs when the server transitions from lobby state to game state
pub fn leave_lobby_system (
    query: Query<&NetPlayer>
) {
    for player in query.iter() {
        if let Err(e) = bincode::serialize_into(&player.stream, &FromServer::StartGame) {
            println!("Leave lobby message failed to send {e}");
            // TODO: might need to disconnect client here, or return to lobby?
        }
    }
}

pub fn lobby_network_system(
    query: Query<(Entity, &NetPlayer, &Agent)>, 
    mut events: EventWriter<PlayerCountChange>, 
    mut commands: Commands,
    mut agent_tracker: ResMut<AgentTracker>,
) {
    for (entity, player, agent) in query.iter() {
        match bincode::deserialize_from::<&TcpStream, FromClient>(&player.stream) {
            Ok(lobby_update) => {
                handle_lobby_update(
                    lobby_update, 
                    agent, 
                    &mut commands, 
                );
            },
            Err(e) => {
                handle_lobby_update_error(
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

fn handle_lobby_update(
    lobby_update: FromClient, 
    agent: &Agent, 
    commands: &mut Commands
) {
    match lobby_update {
        FromClient::Connect{..} => {
            println!("Client sent a second connect message?");
        }
        FromClient::StartGame => {
            if agent.turn_id == 0 {
                commands.insert_resource(NextState(GameState::InGame));
                println!("Should start the game!");
            } else {
                println!("Non-lobby leader sent start game message");
            }
        }
    }
}

// Checks if error is just non-blocking error
fn handle_lobby_update_error(
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




