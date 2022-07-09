use dos_shared::*;
use super::GameState;
use super::MultiplayerState;

use std::net::TcpStream;
use std::io;

use bevy::prelude::*;
use iyes_loopless::prelude::*;



// Recieves and handles messages from the server
pub fn lobby_network_system(mut mp_state: ResMut<MultiplayerState>, mut commands: Commands) {
    let stream =
        match &mp_state.stream {
            None => return,
            Some(i) => i,
    };
    
    match bincode::deserialize_from::<&TcpStream, LobbyUpdateServer>(stream) {
        Ok(lobby_update) => {
            handle_lobby_update(&mut mp_state, lobby_update, &mut commands);
        },
        Err(e) => {
            handle_lobby_update_error(&mut mp_state,e, );
        }
    }
}

// Adjusts multiplayer state based on server message
fn handle_lobby_update(
    mp_state: &mut ResMut<MultiplayerState>, 
    lobby_update: LobbyUpdateServer,
    commands: &mut Commands,
) {
    match lobby_update {
        LobbyUpdateServer::CurrentPlayers{player_names, turn_id} => {
            println!("GOT UPDATE: {:?}",player_names);
            mp_state.player_names = player_names;
            mp_state.turn_id = turn_id;
        }

        LobbyUpdateServer::Disconnect => {
            mp_state.set_disconnected();
        }
        LobbyUpdateServer::StartGame => {
            commands.insert_resource(NextState(GameState::InGame));
        }
    }
}

// Checks if error is just non-blocking error
// Otherwise disconnects
fn handle_lobby_update_error(mp_state: &mut ResMut<MultiplayerState>, e: Box<bincode::ErrorKind>) {
    match *e {
        bincode::ErrorKind::Io(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
        _ => {
            println!("Message receive error: {}", e);
            println!("Disconnecting!");

            mp_state.set_disconnected();
        }
    }
}

// Signals the server to start the game
pub fn send_start_game (stream: Option<&TcpStream>)  {
    if let Some(stream) = stream {
        if let Err(e) = bincode::serialize_into(stream, &LobbyUpdateClient::StartGame) {
            println!("{e}");
        }
    }
}



