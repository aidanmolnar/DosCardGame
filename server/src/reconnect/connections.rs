use dos_shared::{messages::{self, lobby::FromServer}, GameState};
use crate::{multiplayer::AgentTracker, ConnectionTask, game::ServerGame};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use futures_lite::future;
use std::{io::Write, net::TcpStream};


pub fn playercount_change_system(
    mut agent_tracker: ResMut<AgentTracker>,
    mut game: ServerGame,
) {
    if agent_tracker.were_players_updated() {
        println!("Player Count Changed");
        
        let names = agent_tracker.names();

        let mut new_disconnects = Vec::new();
        let mut new_resyncs = Vec::new();

        // Update all the players about the current lobby state
        for (temp_id, (player, mut stream)) 
        in agent_tracker.iter_ids_and_streams()
        .filter(|(player,_)|!agent_tracker.is_disconnected(*player)).enumerate() {

            //----- Update lobby player count
            let lobby_update_message = bincode::serialize(
                &messages::lobby::FromServer::CurrentPlayers{
                    player_names: names.clone(), 
                    turn_id: temp_id as u8
                }
            ).expect("Failed to serialize message");
            
            if let Err(e) =  stream.write_all(&lobby_update_message) {
                println!("Error sending message {player}: {e}");
                new_disconnects.push(player);
            } 

            //----- Update player state if desynced
            if agent_tracker.is_desynced(player) {
                let message = bincode::serialize(
                    &messages::lobby::FromServer::Reconnect(game.get_snapshot(player))
                ).expect("Failed to serialize message!");
        
                if let Err(e) = stream.write_all(&message) {
                    println!("Error sending message to {player}: {e}");
                    new_disconnects.push(player);
                } else {
                    new_resyncs.push(player);
                } 
            }   
        }

        //-----  Handle new changes
        for player in new_resyncs {
            agent_tracker.resync_player(player);
        }

        agent_tracker.reset_players_updated();

        for player in new_disconnects {
            agent_tracker.disconnect_player(player);
        }


        //----- Resume the game if conditions met
        if agent_tracker.all_connected() {
            println!("All reconnected. Resuming game.");
            game.commands.insert_resource(NextState(GameState::InGame));
        } else if agent_tracker.all_disconnected() {
            println!("All disconnected. Resetting server.");
            game.commands.init_resource::<AgentTracker>();
            game.commands.insert_resource(NextState(GameState::MainMenu));
        }

    }
}

pub fn handle_connection_task(
    mut tasks: Query<(Entity, &mut ConnectionTask)>,
    mut agent_tracker: ResMut<AgentTracker>,
    mut commands: Commands,
) {
    for (entity, mut task) in tasks.iter_mut() {

        if let Some(connection_result) = future::block_on(future::poll_once(&mut task.0)) {
            if let Ok(connection_info) = connection_result {
                
                if let Some(player) = agent_tracker.player_id_from_name(&connection_info.name) {
                    if !agent_tracker.is_connected(player) {
                        agent_tracker.reconnect_player(player, connection_info.stream)
                    } else {
                        send_reject_message(connection_info.stream);
                    }
                } else {
                    send_reject_message(connection_info.stream);
                }
            }

            commands.entity(entity).despawn();
        }
    }
}


fn send_reject_message(mut stream: TcpStream) {
    let bytes = bincode::serialize(&FromServer::Reject{
        reason: "Invalid player name".to_string()
    }).expect("Failed to serialize message");
    if let Err(e) = stream.write_all(&bytes) {
        println!("Message failed to send {e}");
    }
}

pub fn on_leave_system(
    mut agent_tracker: ResMut<AgentTracker>,
) {
    let mut disconnects = Vec::new();
        
    for (player, mut stream) in agent_tracker.iter_ids_and_streams() {
        let bytes = bincode::serialize(
            &FromServer::StartGame
        ).expect("Failed to serialize message");

        if let Err(e) =  stream.write_all(&bytes) {
            println!("Message failed to send {e}");
            disconnects.push(player);
        }
    }

    for player in disconnects {
        agent_tracker.disconnect_player(player);
    }
}