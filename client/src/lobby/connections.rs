use dos_shared::{*, channel_config::connection_config};
use super::ui::UiState;

use bevy::{prelude::*, app::AppExit};
use bevy_renet::renet::{RenetClient, RenetError, ClientAuthentication};
use bevy::tasks::Task;

use std::{net::{UdpSocket, SocketAddr}, time::SystemTime};
use futures_lite::future;

// Copies a string into a fixed length byte array.  Truncates if too long, pads if too short
fn copy_from_str(dest:&mut [u8], src:&str){
    // TODO:: fix this clippy warning
    if dest.len() == src.len(){
        dest.copy_from_slice(src.as_bytes());
    } else if dest.len() > src.len(){
        dest[..src.len()].copy_from_slice(src.as_bytes());
    } else {
        dest.copy_from_slice(&src.as_bytes()[..dest.len()]);
    }
}

// Attempts to establish a connection with a given network address
// Sends the player name if successful
pub fn create_connection_task(address: SocketAddr, name: &str) -> Result<RenetClient, RenetError> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    let connection_config = connection_config();

    let mut bytes = [0; 256];
    copy_from_str(&mut bytes, name);
    let user_data = Some(bytes);
    
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr: address,
        user_data,
    };

    RenetClient::new (
        current_time, 
        socket, 
        client_id,
        connection_config, 
        authentication
    )
}


#[derive(Component)]
pub struct ConnectionTask(pub Task<Result<RenetClient, RenetError>>);

// Updates the state of the game with the result of a connection attempt
pub fn handle_connection_task(
    mut transform_tasks: Query<(Entity, &mut ConnectionTask)>,
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
) {
    for (entity, mut task) in transform_tasks.iter_mut() {
        if let Some(connection_response) = future::block_on(future::poll_once(&mut task.0)) {

            match connection_response {
                Ok(client) => {
                    // TODO: client never actually checks if connected to server only if it disconnects
                    // can join lobby even when server isn't running
                    dbg!(client.is_connected());
                    commands.insert_resource(client);
                    ui_state.set_connected();
                }
                Err(e) => {
                    println!("{e}");
                    ui_state.set_disconnected(format!("Connection Failed: {e}"));
                }
            }

            commands.entity(entity).despawn();
        }
    }
}

pub fn exit_system(
    renet_client: Option<ResMut<RenetClient>>,
    events: EventReader<AppExit>,
) {
    if !events.is_empty() {
        if let Some(mut renet_client) = renet_client {
            renet_client.disconnect();
        }
    }
}