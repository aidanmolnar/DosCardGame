use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::io;

use bevy::prelude::*;
use bevy::ecs::event::Events;
use iyes_loopless::prelude::*;

use dos_shared::*;

use::bincode;

/// Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    InGame,
}

#[derive(Default)]
struct MultiplayerState {
    streams: Vec<TcpStream>,
    names: Vec<String>,
}

#[derive(Debug)]
struct ConnectEvent {}

#[derive(Debug)]
struct DisconnectEvent {}

fn main() {

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    App::new()
        .add_plugins(MinimalPlugins)

        .init_resource::<MultiplayerState>()
        .insert_resource(listener) // TODO: How to integrate this with iyes?? Deallocate once in game??

        .add_event::<ConnectEvent>()
        .add_event::<DisconnectEvent>()

        .add_loopless_state(GameState::MainMenu)
        .add_system(check_for_disconnects)
        .add_system(connect_event_listener)

         // Main menu systems
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(listen_for_connections)
                .into()
        )
        

        .run()
}

fn connect_event_listener(mut events: EventReader<ConnectEvent>) {
    for event in events.iter() {
        println!("EVENT: {:?}", event);
    }
}

// yeah ecactly. Handle disconnects by realizing the client hasn't responded correctly... during read/write
//might evolve into a larger function for hnadling netowrk updates in general
fn check_for_disconnects(mut mp_state: ResMut<MultiplayerState>) {

    let mut to_remove = Vec::new();

    let mut data = [0];
    for (i, stream) in mp_state.streams.iter().enumerate() {
        match stream.peek(&mut data) {
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
            }
            Err(e) if e.kind() == io::ErrorKind::ConnectionAborted => {
                if let Err(e2) = stream.shutdown(Shutdown::Both) {
                    println!("Error shutting down tcpstream: {:?}",e2)
                }

                to_remove.push(i);
                println!("Connection Aborted: {:?}",e)
            }

            Err(e) => {
                println!("Hello? {:?}",e.kind())
            }
            Ok(_) => {}
        }
    }

    // Reverse so that indices are accurate positions
    to_remove.reverse();

    for i in to_remove {
        mp_state.streams.remove(i);
    }

}

fn listen_for_connections(listener: Res<TcpListener>, mut mp_state: ResMut<MultiplayerState>, events: EventWriter<ConnectEvent>) {
    // accept connections and process them
    //println!("Server listening on port 3333");
    
    match listener.accept() {
        Ok(connection) => {
            let stream = connection.0;

            if let Err(e) = connect(&mut mp_state, stream, events) {
                println!("Error: {}", e);
            }
        }
        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
        }
        Err(e) => {
            println!("Error: {}", e);
            /* connection failed */
        }
    }
    

    // TODO: May be unnecessary
    //commands.remove_resource::<TcpListener>();
}

fn connect(mp_state: &mut ResMut<MultiplayerState>, stream: TcpStream, mut events: EventWriter<ConnectEvent>) -> Result<(), Box<dyn std::error::Error>> {
    println!("New connection: {}", stream.peer_addr().unwrap());

    // let mut events = Events::<ConnectEvent>::default();
    events.send(ConnectEvent{});

    let client_connect = bincode::deserialize_from::<&TcpStream, ClientConnect>(&stream)?;
    println!("Client name: {:?}",client_connect);

    mp_state.names.push(client_connect.name);
    mp_state.streams.push(stream);

    let x = LobbyUpdate::PlayerCount{players: mp_state.names.clone()};
    let data = bincode::serialize(&x).unwrap();

    for mut stream in &mp_state.streams {
        stream.write_all(&data)?;
    }

    Ok(())
}
