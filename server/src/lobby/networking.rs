use bevy_renet::renet::RenetServer;
use dos_shared::channel_config::LOBBY_CHANNEL_ID;
use dos_shared::messages::lobby::*;
use super::GameState;
use super::multiplayer::MultiplayerState;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

// Runs when the server transitions from lobby state to game state
pub fn leave_lobby_system (
    mut renet_server: ResMut<RenetServer>,
) {
    let message = 
        bincode::serialize(&FromServer::StartGame)
        .expect("Failed to serialize message.");

    renet_server.broadcast_message(LOBBY_CHANNEL_ID, message);
}

pub fn lobby_network_system(
    mut commands: Commands,
    mp_state: Res<MultiplayerState>,
    mut renet_server: ResMut<RenetServer>,
) {
    for client_id in renet_server.clients_id().into_iter() {
        while let Some(message) = renet_server.receive_message(client_id, LOBBY_CHANNEL_ID) {

            let player = mp_state.player_from_renet_id(client_id);

            // TODO: don't expect
            let update = bincode::deserialize(&message)
            .expect("Couldn't deserialize message"); 
            // Handle each stream

            handle_lobby_update(
                update, 
                player,
                &mut commands, 
            );
        }
    }
}


fn handle_lobby_update(
    lobby_update: FromClient, 
    player: usize,
    commands: &mut Commands
) {
    dbg!(lobby_update.clone());

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
