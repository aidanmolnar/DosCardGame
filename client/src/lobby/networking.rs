use dos_shared::{
    net_config::LOBBY_CHANNEL_ID, 
    messages::lobby::{FromClient, FromServer}
};

use crate::game::CallDos;
use super::{GameState, MultiplayerState, ui::UiState};

use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use iyes_loopless::prelude::*;

// Recieves and handles messages from the server
pub fn lobby_network_system(
    mut renet_client: ResMut<RenetClient>,
    mut mp_state: ResMut<MultiplayerState>, 
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
    game_state: Res<CurrentState<GameState>>,
) { 
    // Handle loss of connection
    if !renet_client.is_connected() {
        // Return to the main menu if not already there
        if game_state.0 != GameState::MainMenu {
            commands.insert_resource(NextState(GameState::MainMenu));
        }

        if renet_client.disconnected().is_some() {
            mp_state.disconnect();
            ui_state.set_disconnected("Connection timed out".into());
            commands.remove_resource::<RenetClient>();
        }

        return;
    }
    
    // Read all lobby messages from server
    while let Some(message) = renet_client.receive_message(LOBBY_CHANNEL_ID) {
        
        let update = bincode::deserialize::<FromServer>(&message)
        .expect("Failed to deserialize message!");

        handle_lobby_update(
            &mut mp_state, 
            &mut ui_state, 
            &mut commands, 
            game_state.clone().0,
            update
        );
    }
}

// Adjusts multiplayer state based on server message
fn handle_lobby_update(
    mp_state: &mut ResMut<MultiplayerState>, 
    ui_state: &mut ResMut<UiState>, 
    commands: &mut Commands,
    game_state: GameState,
    lobby_update: FromServer,
) {
    match lobby_update {
        FromServer::CurrentPlayers{player_names, turn_id} => {
            mp_state.connect(player_names);
            mp_state.turn_id = turn_id as usize;

            // Update to current players during game means someone disconnected
            if game_state == GameState::InGame {
                commands.insert_resource(NextState(GameState::Reconnect));
            }
        }
        FromServer::StartGame => {
            commands.insert_resource(NextState(GameState::InGame));
        }
        FromServer::Reject { reason } => {
            commands.remove_resource::<RenetClient>();
            ui_state.set_disconnected(reason);
        },
        FromServer::Reconnect(game_snapshot) => {
            if game_snapshot.dos.is_some() {
                commands.init_resource::<CallDos>();
            }
            commands.insert_resource(game_snapshot);
            commands.insert_resource(NextState(GameState::Reconnect));
        },
    }
    
}


// Send signal to server to start the game
pub fn send_start_game (renet_client: &mut RenetClient)  {
    let message = bincode::serialize(&FromClient::StartGame).expect("Failed to serialize message");
    renet_client.send_message(LOBBY_CHANNEL_ID, message);
}



