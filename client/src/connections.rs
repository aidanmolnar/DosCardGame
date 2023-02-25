use dos_shared::net_config::{connection_config, PROTOCOL_ID};

use bevy::{app::AppExit, prelude::*};
use bevy_renet::renet::{ClientAuthentication, RenetClient, RenetError};

use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

// Copies a string into a fixed length byte array.  Truncates if too long, pads if too short
fn copy_from_str(dest: &mut [u8], src: &str) {
    match dest.len().cmp(&src.len()) {
        std::cmp::Ordering::Equal => dest.copy_from_slice(src.as_bytes()),
        std::cmp::Ordering::Less => dest.copy_from_slice(&src.as_bytes()[..dest.len()]),
        std::cmp::Ordering::Greater => dest[..src.len()].copy_from_slice(src.as_bytes()),
    }
}

// Attempts to establish a connection with a given network address
// Sends the player name if successful
pub fn new_renet_client(address: SocketAddr, name: &str) -> Result<RenetClient, RenetError> {
    // Convert the name to a byte array to send to server
    let mut bytes = [0; 256];
    copy_from_str(&mut bytes, name);
    let user_data = Some(bytes);

    // Generate a client id from system time
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    #[allow(clippy::cast_possible_truncation)] // Truncation is intended
    let client_id = current_time.as_millis() as u64;

    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr: address,
        user_data,
    };

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let connection_config = connection_config();

    RenetClient::new(current_time, socket, connection_config, authentication)
}

// Gracefully disconnects when closing the app instead of relying on timeout
pub fn exit_system(renet_client: Option<ResMut<RenetClient>>, events: EventReader<AppExit>) {
    if !events.is_empty() {
        if let Some(mut renet_client) = renet_client {
            renet_client.disconnect();
        }
    }
}
