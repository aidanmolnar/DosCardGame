use dos_shared::*;

use super::multiplayer::AgentTracker;

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::tasks::Task;

use std::net::{TcpListener, TcpStream};
use std::io;

pub struct ConnectionListeningPlugin;

impl Plugin for ConnectionListeningPlugin {
    fn build(&self, app: &mut App) {
        let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
        listener.set_nonblocking(true).expect("Cannot set non-blocking on listener!");

        app
        .init_resource::<AgentTracker>()
        .insert_resource(listener) 
        
        .add_system(
            listen_for_connections
        );
    }
}

// TODO: break up / simplify this function
fn listen_for_connections(
    listener: Res<TcpListener>, 
    mut commands: Commands, 
) {
    let thread_pool = AsyncComputeTaskPool::get();
    // accept connections and process them
    match listener.accept() {
        Ok(connection) => {
            let stream = connection.0;

            let task = thread_pool.spawn(async move {
                create_connection_task(stream)
            });
            commands.spawn().insert(ConnectionTask(task));
        }
        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
        }
        Err(e) => {
            println!("Connection failed: {}", e);
        }
    }
}

#[derive(Component)]
pub struct ConnectionTask(
    pub Task<Result<ConnectionInfo, ()>>
);

pub struct ConnectionInfo {
    pub name: String,
    pub stream: TcpStream,
}

// TODO: Add errors to result
fn create_connection_task(stream: TcpStream) -> Result<ConnectionInfo, ()> {
    println!("New connection: {}", stream.peer_addr().unwrap());

    let client_connect = match bincode::deserialize_from::<&TcpStream, messages::lobby::FromClient>(&stream) {
        Ok(c) => {c}
        Err(e) => {
            println!("Aborting new connection: {e}");
            stream.shutdown(std::net::Shutdown::Both).expect("Couldn't close stream!");
            return Err(())
        }
    };
    println!("Client name: {:?}",client_connect);

    if let messages::lobby::FromClient::Connect {name} = client_connect {
        Ok(ConnectionInfo{name, stream})
    } else {
        Err(())
    }
}
 
