use dos_shared::messages::lobby::*;

use crate::multiplayer::AgentTracker;

use bevy::prelude::*;

use::bincode;
use std::net::TcpStream;
use std::io::ErrorKind;

// If there are no more disconnected players, resume the game.  Send a message to anyone who was disconnected.  Eepers 

pub fn reconnect_network_system(
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
                    // handle_reconnect_update(
                    //     lobby_update, 
                    //     player,
                    //     &mut commands, 
                    // );
                    // TODO: Shouldn't panic.
                    panic!("Received lobby update out of turn: {:?}", lobby_update);
                },
                Err(e) => {
                    handle_reconnect_update_error(
                        e, 
                        player, 
                        &mut agent_tracker,
                    );
                }
            }

        }
    }
}


/* fn handle_reconnect_update(
    lobby_update: FromClient, 
    player: usize,
    commands: &mut Commands
) {
    //todo!();
    /* match lobby_update {
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
    } */
} */

// Checks if error is just non-blocking error
fn handle_reconnect_update_error(
    e: Box<bincode::ErrorKind>,
    player: usize,
    agent_tracker: &mut ResMut<AgentTracker>
) {
    match *e {
        bincode::ErrorKind::Io(ref e) if e.kind() == ErrorKind::WouldBlock => {}
        _ => {
            println!("Message receive error: {}", e);

            agent_tracker.disconnect_player(player);
        }
    }
}