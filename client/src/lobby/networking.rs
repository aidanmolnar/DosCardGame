use dos_shared::channel_config::LOBBY_CHANNEL_ID;
use dos_shared::messages::lobby::*;

use super::GameState;
use super::MultiplayerState;
use super::UiState;
use crate::game::CallDos;

use bevy_renet::renet::RenetClient;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

// Recieves and handles messages from the server
pub fn lobby_network_system(
    mut renet_client: ResMut<RenetClient>,
    mut mp_state: ResMut<MultiplayerState>, 
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
    game_state: Res<CurrentState<GameState>>,
) {    
    if renet_client.disconnected().is_some() {
        mp_state.disconnect();
        commands.remove_resource::<RenetClient>();
        return;
    }

    while let Some(message) = renet_client.receive_message(LOBBY_CHANNEL_ID) {
        
        let update = bincode::deserialize::<FromServer>(&message)
        .expect("Failed to deserialize message!");

        dbg!(update.clone());

        handle_lobby_update(
            &mut mp_state, 
            &mut ui_state, 
            &mut commands, 
            &game_state.clone().0,
            update
        )
    }
}

// Adjusts multiplayer state based on server message
fn handle_lobby_update(
    mp_state: &mut ResMut<MultiplayerState>, 
    ui_state: &mut ResMut<UiState>, 
    commands: &mut Commands,
    game_state: &GameState,
    lobby_update: FromServer,
) {
    match lobby_update {
        FromServer::CurrentPlayers{player_names, turn_id} => {
            println!("GOT UPDATE: {:?}",player_names);
            mp_state.connect(player_names);
            mp_state.turn_id = turn_id as usize;

            if *game_state == GameState::InGame {
                commands.insert_resource(NextState(GameState::Reconnect));
            }
        }
        FromServer::StartGame => {
            commands.insert_resource(NextState(GameState::InGame));
        }
        FromServer::Reject { reason } => {
            println!("Disconnecting!");
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


// Signals the server to start the game
pub fn send_start_game (renet_client: &mut RenetClient)  {
    let message = bincode::serialize(&FromClient::StartGame).expect("Failed to serialize message");
    renet_client.send_message(LOBBY_CHANNEL_ID, message);
}



