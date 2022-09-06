
use dos_shared::messages;

use crate::{multiplayer::AgentTracker, ConnectionTask};

use bevy::prelude::*;

use futures_lite::future;


// TODO: Rename this to something less wordy and more descriptive
// TODO: This only works in lobby... Players must be ready to receive lobby updates
pub fn playercount_change_system(
    mut agent_tracker: ResMut<AgentTracker>,
) {
    if agent_tracker.were_players_updated() {
        println!("Player Count Changed");

        agent_tracker.remove_disconnected_players();
        
        let names = agent_tracker.names();

        let mut new_disconnects = Vec::new();

        // Update all the players about the current lobby state
        for (player, stream) in agent_tracker.iter_ids_and_streams() {

            if let Err(e) = bincode::serialize_into(stream, 
                &messages::lobby::FromServer::CurrentPlayers{
                    player_names: names.clone(), 
                    turn_id: player as u8}) 
            {
                println!("Error sending message {player}: {e}");
                new_disconnects.push(player);
            }
            
        }

        agent_tracker.reset_players_updated();

        for player in new_disconnects {
            agent_tracker.disconnect_player(player);
        }

        
    }
}

pub fn handle_connection_task(
    mut tasks: Query<(Entity, &mut ConnectionTask)>,
    mut commands: Commands,
    mut agent_tracker: ResMut<AgentTracker>,
) {
    for (entity, mut task) in tasks.iter_mut() {

        if let Some(connection_result) = future::block_on(future::poll_once(&mut task.0)) {
            if let Ok(connection_info) = connection_result {
                println!("New player connected");
                agent_tracker.new_player(connection_info.name, connection_info.stream);
            }

            commands.entity(entity).despawn();
        }
    }
}

