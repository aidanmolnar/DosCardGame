use dos_shared::messages::lobby::*;
use super::GameState;
use super::multiplayer::AgentTracker;
use super::connection_listening::{PlayerCountChange, disconnect};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use::bincode;
use std::net::TcpStream;
use std::io;

// Runs when the server transitions from lobby state to game state
pub fn leave_lobby_system (
    agent_tracker: Res<AgentTracker>,
) {
    for stream in agent_tracker.iter_streams() {
        if let Err(e) = bincode::serialize_into(stream, &FromServer::StartGame) {
            println!("Leave lobby message failed to send {e}");
            // TODO: might need to disconnect client here, or return to lobby?
        }
    }
}

pub fn lobby_network_system(
    mut events: EventWriter<PlayerCountChange>, 
    mut commands: Commands,
    agent_tracker: Res<AgentTracker>,
) {
    for (player, stream) in agent_tracker.iter_ids_and_streams() {
        match bincode::deserialize_from::<&TcpStream, FromClient>(stream) {
            Ok(lobby_update) => {
                handle_lobby_update(
                    lobby_update, 
                    player,
                    &mut commands, 
                );
            },
            Err(e) => {
                handle_lobby_update_error(
                    e, 
                    player, 
                    &mut events, 
                );
            }
        }
    }
}


fn handle_lobby_update(
    lobby_update: FromClient, 
    player: usize,
    commands: &mut Commands
) {
    match lobby_update {
        FromClient::Connect{..} => {
            println!("Client sent a second connect message?");
        }
        FromClient::StartGame => {
            if player == 0 {
                commands.insert_resource(NextState(GameState::InGame));
                println!("Should start the game!");
            } else {
                // TODO: remove panic
                panic!("Non-lobby leader sent start game message");
            }
        }
    }
}

// Checks if error is just non-blocking error
fn handle_lobby_update_error(
        e: Box<bincode::ErrorKind>,
        player: usize,
        events: &mut EventWriter<PlayerCountChange>, 
) {
    match *e {
        bincode::ErrorKind::Io(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
        _ => {
            println!("Message receive error: {}", e);

            disconnect(player, events);
        }
    }
}




