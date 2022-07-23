use dos_shared::messages::lobby::*;
use super::GameState;
use super::MultiplayerState;
use super::UiState;

use std::net::TcpStream;
use std::io;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

// Recieves and handles messages from the server
pub fn lobby_network_system(
    mut mp_state: ResMut<MultiplayerState>, 
    mut ui_state: ResMut<UiState>,
    mut commands: Commands
) {
    let stream =
        match &mp_state.stream {
            None => return,
            Some(i) => i,
    };
    
    match bincode::deserialize_from::<&TcpStream, FromServer>(stream) {
        Ok(lobby_update) => {
            handle_lobby_update(&mut mp_state, lobby_update, &mut commands);
        },
        Err(e) => {
            handle_lobby_update_error(
                &mut mp_state,
                &mut ui_state,
                e, 
            );
        }
    }
}

// Adjusts multiplayer state based on server message
fn handle_lobby_update(
    mp_state: &mut ResMut<MultiplayerState>, 
    lobby_update: FromServer,
    commands: &mut Commands,
) {
    match lobby_update {
        FromServer::CurrentPlayers{player_names, turn_id} => {
            println!("GOT UPDATE: {:?}",player_names);
            mp_state.player_names = player_names;
            mp_state.turn_id = turn_id as usize;
        }
        FromServer::Disconnect => {
            mp_state.set_disconnected();
        }
        FromServer::StartGame => {
            commands.insert_resource(NextState(GameState::InGame));
        }
    }
}

// Checks if error is just non-blocking error
// Otherwise disconnects
fn handle_lobby_update_error(
    mp_state: &mut ResMut<MultiplayerState>, 
    ui_state: &mut ResMut<UiState>,
    e: Box<bincode::ErrorKind>
) {
    match *e {
        bincode::ErrorKind::Io(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
        _ => {
            println!("Message receive error: {}", e);
            println!("Disconnecting!");

            mp_state.set_disconnected();
            ui_state.set_disconnected("Connection Terminated");
        }
    }
}

// Signals the server to start the game
pub fn send_start_game (stream: Option<&TcpStream>)  {
    if let Some(stream) = stream {
        if let Err(e) = bincode::serialize_into(stream, &FromClient::StartGame) {
            println!("{e}");
        }
    }
}



