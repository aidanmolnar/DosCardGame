use dos_shared::{messages::{self, game::GameAction, lobby::FromServer}, GameState};
use futures_lite::future;
use crate::{multiplayer::AgentTracker, ConnectionTask};

use bevy::prelude::*;
use iyes_loopless::state::NextState;

use std::io::Write;

pub fn playercount_change_system(
    mut agent_tracker: ResMut<AgentTracker>,
    mut commands: Commands,
) {
    if agent_tracker.were_players_updated() {
        println!("Player Count Changed");
        
        let names = agent_tracker.names();

        let disconnect_message = bincode::serialize(
            &messages::game::FromServer {
                action: GameAction::DisconnectOccurred,
                conditions: Vec::new(),
                cards: Vec::new(),
            }
        ).expect("Failed to serialize message");

        let mut new_disconnects = Vec::new();

        let mut temp_id: u8 = 0;
        
        // Update all the players about the current lobby state
        for (player, mut stream) in agent_tracker.iter_ids_and_streams(){
            if !agent_tracker.is_connected(player) {
                continue;
            }

            if let Err(e) =  stream.write_all(&disconnect_message) {
                println!("Error sending message {player}: {e}");
                new_disconnects.push(player);
            } 

            let lobby_update_message = bincode::serialize(
                &messages::lobby::FromServer::CurrentPlayers{
                    player_names: names.clone(), 
                    turn_id: temp_id
                }
            ).expect("Failed to serialize message");

            temp_id += 1;

            if let Err(e) =  stream.write_all(&lobby_update_message) {
                println!("Error sending message {player}: {e}");
                new_disconnects.push(player);
            } 
        }

        agent_tracker.reset_players_updated();

        for player in new_disconnects {
            agent_tracker.disconnect_player(player);
        }

        commands.insert_resource(NextState(GameState::Reconnect));
    }
}

pub fn handle_connection_task(
    mut tasks: Query<(Entity, &mut ConnectionTask)>,
    mut commands: Commands,
) {
    for (entity, mut task) in tasks.iter_mut() {

        if let Some(connection_result) = future::block_on(future::poll_once(&mut task.0)) {
            if let Ok(mut connection_info) = connection_result {
                
                let bytes = bincode::serialize(&FromServer::Reject{
                    reason: "Game already in progress!".to_string()
                }).expect("Failed to serialize message");
    
                if let Err(e) =  connection_info.stream.write_all(&bytes) {
                    println!("Message failed to send {e}");
                }

            }

            commands.entity(entity).despawn();
        }
    }
}
