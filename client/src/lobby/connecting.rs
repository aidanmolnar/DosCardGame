use dos_shared::*;

use super::MultiplayerState;
use super::ui::UiState;


use bevy::prelude::*;
use bevy::tasks::Task;

use std::net::TcpStream;
use std::io;
use futures_lite::future;

// TODO: move to child of ui?

// Attempts to establish a connection with a given network address
// Sends the player name if successful
pub fn create_connection_task(address: &str, name: &str) -> Result<TcpStream, io::Error> {
    match TcpStream::connect(address) {
        Ok(stream) => {
            println!("Successfully connected to server {address}");

            // Immediately send the client info (name)
            if bincode::serialize_into(&stream, &LobbyUpdateClient::Connect{name: name.to_string()}).is_err() {
                println!("Failed to send message");
                return Err(io::Error::new(io::ErrorKind::Other, "Failed to send message"));
            }
     
            stream.set_nonblocking(true).expect("nonblocking failure");
 
            Ok(stream)
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
            Err(e)
        }
    }
}

// Updates the state of the game with the result of a connection attempt
pub fn handle_connection_task(
    mut transform_tasks: Query<(Entity, &mut Task<Result<TcpStream, io::Error>>)>,
    mut commands: Commands,
    mut mp_state: ResMut<MultiplayerState>,
    mut ui_state: ResMut<UiState>,
) {
    for (entity, mut task) in transform_tasks.iter_mut() {

        if let Some(connection_response) = future::block_on(future::poll_once(&mut *task)) {

            match connection_response {
                Ok(stream) => {
                    mp_state.set_connected(stream);
                    ui_state.set_connected();
                }
                Err(e) => {
                    println!("{e}");
                    mp_state.set_disconnected();
                    ui_state.set_disconnected("Connection Failed");
                }
            }

            commands.entity(entity).despawn();
        }
    }
}