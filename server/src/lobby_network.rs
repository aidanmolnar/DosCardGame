use std::net::{TcpListener, TcpStream};
use std::io::{Write};
use std::io;

use bevy::prelude::*;

use dos_shared::*;

use::bincode;

#[derive(Default)]
pub struct MultiplayerState {
    pub streams: Vec<TcpStream>,
    pub player_names: Vec<String>,
}


pub fn lobby_network_system(mut mp_state: ResMut<MultiplayerState>) {

    let mut to_remove = Vec::new();

    for (i, stream) in mp_state.streams.iter().enumerate() {
        match bincode::deserialize_from::<&TcpStream, LobbyUpdateClient>(stream) {
            Ok(lobby_update) => {
                //println!("{:?}", lobby_update);

                match lobby_update {
                    LobbyUpdateClient::Disconnect => {
                        println!("{:?} disconnected", &mp_state.player_names.get(i));
                        to_remove.push(i);
                    }
                    LobbyUpdateClient::Connect{..} => {
                        println!("Client sent a second connect message?");
                    }
                    LobbyUpdateClient::StartGame => {
                        if i == 0 {
                            println!("Should start the game!");
                        } else {
                            println!("Non-lobby leader sent start game message");
                        }
                    }
                }

            },
            Err(e) => {
                handle_lobby_update_error(e);
            }
        }
    }

    if !to_remove.is_empty() {
        // We need to remove in reverse order otherwise later indicies will be off because array has shrunk
        to_remove.reverse();

        for i in &to_remove {
            mp_state.player_names.remove(*i);
            mp_state.streams.remove(*i);
        }

        if to_remove.contains(&0) {
            if let Some(first_stream) = mp_state.streams.get(0) {
                // TODO: Shouldn't panic
                bincode::serialize_into(first_stream, &LobbyUpdateServer::YouAreLobbyLeader).expect("uh oh");
            }
        }

        // TODO: Shouldn't panic
        send_current_players_update(&mut mp_state).expect("uh oh");
    }
}

// Checks if error is just non-blocking error
fn handle_lobby_update_error(e: Box<bincode::ErrorKind>) {
    match *e {
        bincode::ErrorKind::Io(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
        _ => {
            println!("Message receive error: {}", e);

            //disconnect(mp_state);
        }
    }
}


pub fn listen_for_connections(listener: Res<TcpListener>, mut mp_state: ResMut<MultiplayerState>) {
    // accept connections and process them
    //println!("Server listening on port 3333");
    
    match listener.accept() {
        Ok(connection) => {
            let stream = connection.0;

            if let Err(e) = connect(&mut mp_state, stream) {
                println!("Error: {}", e);
                //panic!("{e}")
            }
        }
        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
        }
        Err(e) => {
            println!("Error: {}", e);
            //panic!("{e}")
            /* connection failed */
        }
    }
}

// TODO: this should be in a thread so one slow/malicious client connecting doesn't stall the whole server
// Would need to wrap mp state somehow or defer state updates?
fn connect(mp_state: &mut ResMut<MultiplayerState>, stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("New connection: {}", stream.peer_addr().unwrap());

    let client_connect = bincode::deserialize_from::<&TcpStream, LobbyUpdateClient>(&stream)?;
    println!("Client name: {:?}",client_connect);

    if let LobbyUpdateClient::Connect {name} = client_connect {
        if mp_state.streams.is_empty() {
            bincode::serialize_into(&stream, &LobbyUpdateServer::YouAreLobbyLeader)?;
        }

        mp_state.player_names.push(name);
        mp_state.streams.push(stream);

    } else {
        // TODO: Shouldn't panic
        panic!("Didnt receive update");
    }

    send_current_players_update(mp_state)?;

    Ok(())
}

fn send_current_players_update(mp_state: &mut ResMut<MultiplayerState>) -> Result<(), Box<dyn std::error::Error>>{
    let x = LobbyUpdateServer::CurrentPlayers{player_names: mp_state.player_names.clone()};
    let data = bincode::serialize(&x).unwrap();

    for mut stream in &mp_state.streams {
        stream.write_all(&data)?;
    }

    Ok(())
}
