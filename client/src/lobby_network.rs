use dos_shared::*;
use super::GameState;

use std::net::TcpStream;
use std::io;

use bevy::prelude::*;
use iyes_loopless::prelude::*;


#[derive(Default, Debug)]
pub struct MultiplayerState {
    pub stream: Option<TcpStream>,
    pub player_names: Vec<String>,
    pub is_lobby_leader: bool,
}

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
        LobbyUpdateServer::CurrentPlayers{player_names} => {
            println!("GOT UPDATE: {:?}",player_names);
            mp_state.player_names = player_names;
        }
        LobbyUpdateServer::YouAreLobbyLeader => {
            mp_state.is_lobby_leader = true;
        }
        LobbyUpdateServer::Disconnect => {
            disconnect(mp_state);
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

            disconnect(mp_state);
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

// Attempts to establish a connection with a given network address
// Sends the player name if successful
pub fn connect(address: &str, name: &str) -> Result<TcpStream, Box<dyn std::error::Error>> {
    match TcpStream::connect(address) {
        Ok(stream) => {
            println!("Successfully connected to server {address}");

            // Immediately send the client info (name)
            bincode::serialize_into(&stream, &LobbyUpdateClient::Connect{name: name.to_string()}).expect("sending error");
     
            stream.set_nonblocking(true).expect("nonblocking failure");
 
            Ok(stream)
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
            Err(Box::new(e))
        }
    }
}

// Closes the connection to the server and resets the client's mutliplayer state
pub fn disconnect(mp_state: &mut ResMut<MultiplayerState>) {
    // unwrap stream
    let stream =
        match &mp_state.stream {
            None => return,
            Some(i) => i,
    };

    end_connection(stream);

    // Reset state to default
    mp_state.stream = None;
    mp_state.player_names = Vec::new();
    mp_state.is_lobby_leader = false;
}

// Closes the connection to the server
pub fn end_connection(stream: &TcpStream) {
    // Send a disconnect message to exit gracefully on server-side if possible
    if let Err(e) = bincode::serialize_into(stream, &LobbyUpdateClient::Disconnect{}) {
        println!("Disconnect message send error: {:?}", e);
    }

    if let Err(e) = stream.shutdown(std::net::Shutdown::Both) {
        println!("Exit shutdown error: {:?}", e);
    }
}