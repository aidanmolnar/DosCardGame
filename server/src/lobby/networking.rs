use dos_shared::{
    net_config::LOBBY_CHANNEL_ID, 
    messages::lobby::{FromClient, FromServer}
};

use super::{
    GameState, 
    multiplayer::MultiplayerState
};

use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use iyes_loopless::prelude::*;

// Runs when the server transitions from lobby state to game state
pub fn leave_system (
    mut renet_server: ResMut<RenetServer>,
) {
    let message = 
        bincode::serialize(&FromServer::StartGame)
        .expect("Failed to serialize message.");

    renet_server.broadcast_message(LOBBY_CHANNEL_ID, message);
}

// Receive lobby messages from clients
pub fn network_system(
    mut commands: Commands,
    mp_state: Res<MultiplayerState>,
    mut renet_server: ResMut<RenetServer>,
) {
    for client_id in renet_server.clients_id() {
        while let Some(message) = renet_server.receive_message(client_id, LOBBY_CHANNEL_ID) {
            let player = mp_state.player_from_renet_id(client_id);

            // Remove player if message can't be serialized
            if let Ok(update) = bincode::deserialize(&message) {
                handle_lobby_update(
                    &mut commands, 
                    &mut renet_server, 
                    update, 
                    player,
                    client_id
                );
            } else {
                lobby_disconnect(
                    &mut renet_server, 
                    client_id
                );
            }
        }
    }
}

fn handle_lobby_update(
    commands: &mut Commands,
    renet_server: &mut RenetServer, 
    lobby_update: FromClient, 
    player: usize,
    renet_id: u64
) {
    match lobby_update {
        FromClient::StartGame => {
            if player == 0 {
                commands.insert_resource(NextState(GameState::InGame));
                println!("Should start the game!");
            } else {
                // Only lobby leader should be able to send start game message
                lobby_disconnect(
                    renet_server, 
                    renet_id
                );
            }
        }
    }
}

// Remove a player from the lobby
fn lobby_disconnect(
    renet_server: &mut RenetServer, 
    renet_id: u64
) {
    renet_server.disconnect(renet_id);
    println!("Disconnecting {renet_id}");
}