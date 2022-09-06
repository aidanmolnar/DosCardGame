use dos_shared::messages::lobby::*;
use super::GameState;
use super::multiplayer::AgentTracker;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use::bincode;
use std::net::TcpStream;
use std::io::{self, Write};

// Runs when the server transitions from lobby state to game state
pub fn leave_lobby_system (
    agent_tracker: Res<AgentTracker>,
) {
    let message = bincode::serialize(&FromServer::StartGame).expect("Failed to serialize message.");


    for mut stream in agent_tracker.iter_streams() {
        if let Err(e) = stream.write_all(&message) {
            println!("Leave lobby message failed to send {e}");
            // TODO: might need to disconnect client here, or return to lobby?
        }
    }
}

pub fn lobby_network_system(
    mut commands: Commands,
    mut agent_tracker: ResMut<AgentTracker>,
) {
    // Loop over all the streams
    for player in 0..agent_tracker.num_agents() {
        if let Some(stream) = agent_tracker.try_get_stream(player) {
            if !agent_tracker.is_connected(player) {
                continue;
            }

            // Handle each stream
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
                        &mut agent_tracker,
                    );
                }
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
                // Disconnect offending player
                panic!("Non-lobby leader sent start game message");
            }
        }
    }
}

// Checks if error is just non-blocking error
fn handle_lobby_update_error(
    e: Box<bincode::ErrorKind>,
    player: usize,
    agent_tracker: &mut ResMut<AgentTracker>
) {
    match *e {
        bincode::ErrorKind::Io(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
        _ => {
            println!("Message receive error: {}", e);

            agent_tracker.disconnect_player(player);
        }
    }
}




