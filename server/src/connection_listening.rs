use dos_shared::*;

use super::multiplayer::{MultiplayerState, NetPlayer};
use super::GameState;

use bevy::prelude::*;
use bevy::ecs::event::Events;
use iyes_loopless::prelude::*;

use std::net::{TcpListener, TcpStream};
use std::io;

pub struct ConnectionListeningPlugin;

impl Plugin for ConnectionListeningPlugin {
    fn build(&self, app: &mut App) {
        let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
        listener.set_nonblocking(true).expect("Cannot set non-blocking");

        app
        .init_resource::<MultiplayerState>()
        .insert_resource(listener) // TODO: How to integrate this with iyes?? Deallocate once in game??
        .init_resource::<Events<PlayerCountChange>>()

        .add_system(listen_for_connections
            .run_in_state(GameState::MainMenu)
            .after("handle_changes"));
    }
}

pub struct PlayerCountChange;

pub fn listen_for_connections(listener: Res<TcpListener>, commands: Commands, events: EventWriter<PlayerCountChange>) {
    // accept connections and process them
    //println!("Server listening on port 3333");
    match listener.accept() {
        Ok(connection) => {
            let stream = connection.0;

            connect(commands, events, stream);
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
fn connect(mut commands: Commands, mut events: EventWriter<PlayerCountChange>, stream: TcpStream){
    println!("New connection: {}", stream.peer_addr().unwrap());

    let client_connect = match bincode::deserialize_from::<&TcpStream, LobbyUpdateClient>(&stream) {
        Ok(c) => {c}
        Err(e) => {
            println!("Aborting new connection: {e}");
            stream.shutdown(std::net::Shutdown::Both).expect("Couldn't close stream?");
            return;
    }
    };
    println!("Client name: {:?}",client_connect);

    if let LobbyUpdateClient::Connect {name} = client_connect {

        commands.spawn().insert(
            NetPlayer {
                name,
                stream,
            }
        );
        events.send(PlayerCountChange {  })

    } else {
        // TODO: Shouldn't panic
        panic!("Didnt receive update");
    }

}


pub fn disconnect(
    entity: Entity, 
    player: &NetPlayer,
    events: &mut EventWriter<PlayerCountChange>, 
    commands: &mut Commands
) {
    if let Err(e) = bincode::serialize_into(&player.stream, &LobbyUpdateServer::Disconnect) {
        println!("Disconnect message failed to send {e}");
    }
    events.send(PlayerCountChange{});
    commands.entity(entity).despawn();
}

