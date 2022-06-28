use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io;

use bevy::prelude::*;
use bevy::ecs::event::Events;

use dos_shared::*;

use::bincode;

#[derive(Default)]
pub struct MultiplayerState {
    pub streams: Vec<TcpStream>,
    pub player_names: Vec<String>,
}

#[derive(Component)]
pub struct TurnId {
    pub id: u8,
}

#[derive(Component)]
pub struct NetPlayer {
    pub name: String,
    pub stream: TcpStream,
}

pub struct PlayerCountChange {}


pub fn lobby_network_system(query: Query<(Entity, &NetPlayer, &TurnId)>, mut events: EventWriter<PlayerCountChange>, mut commands: Commands) {
    for (entity, player, turn_id) in query.iter() {
        match bincode::deserialize_from::<&TcpStream, LobbyUpdateClient>(&player.stream) {
            Ok(lobby_update) => {
                //println!("{:?}", lobby_update);

                handle_lobby_update(lobby_update, player, turn_id, entity, &mut events, &mut commands);

            },
            Err(e) => {
                handle_lobby_update_error(e, entity, player, &mut events, &mut commands);
            }
        }
    }
}

// Abysmal name for this function
pub fn handle_playercount_change_system(
    mut query: Query<(Entity, &mut NetPlayer, Option< &mut TurnId>)>, 
    mut events: ResMut<Events<PlayerCountChange>>,
    mut commands: Commands) {
    
    if !events.is_empty() {
        println!("Player count change, reassigning ids");
        events.clear();

        // Sort entities so ids can be reassigned
        let mut entities = query.iter_mut().collect::<Vec<_>>();
        entities.sort_by_key(|e| 
            match &e.2 {
                Some(i) => {i.id}
                None => {255}
            }
        );

        // Reassign ids
        for (i,(entity, _, turn_id_opt))in entities.iter_mut().enumerate() {
            match turn_id_opt {
                Some(turn_id) => {turn_id.id = i as u8},
                None => {
                    commands.entity(*entity).insert(TurnId{id:i as u8});
                }
            }
        }

        // Update all the people
        let names = entities.iter().map(|x| x.1.name.clone()).collect::<Vec<_>>();

        let x = LobbyUpdateServer::CurrentPlayers{player_names: names};
        let data = bincode::serialize(&x).unwrap();

        for (_,mut player,_) in query.iter_mut() {
            player.stream.write_all(&data).expect("RIOT");
        }

        // Send lobby leader notification
    }
    
}


fn handle_lobby_update(
    lobby_update: LobbyUpdateClient, 
    player: &NetPlayer, 
    turn_id: &TurnId, 
    entity: Entity, 
    events: &mut EventWriter<PlayerCountChange>, 
    commands: &mut Commands
) {

    match lobby_update {
        LobbyUpdateClient::Disconnect => {
            println!("{:?} disconnected", player.name);

            disconnect(entity, player, events, commands);
        }
        LobbyUpdateClient::Connect{..} => {
            println!("Client sent a second connect message?");
        }
        LobbyUpdateClient::StartGame => {
            if turn_id.id == 0 {
                println!("Should start the game!");
            } else {
                println!("Non-lobby leader sent start game message");
            }
        }
    }
}

fn disconnect(
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

// Checks if error is just non-blocking error
fn handle_lobby_update_error(
        e: Box<bincode::ErrorKind>,
        entity: Entity,
        player: &NetPlayer,
        events: &mut EventWriter<PlayerCountChange>, 
        commands: &mut Commands) {
    match *e {
        bincode::ErrorKind::Io(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
        _ => {
            println!("Message receive error: {}", e);

            disconnect(entity, player, events, commands);
        }
    }
}


pub fn listen_for_connections(listener: Res<TcpListener>, commands: Commands, events: EventWriter<PlayerCountChange>) {
    // accept connections and process them
    //println!("Server listening on port 3333");
    
    match listener.accept() {
        Ok(connection) => {
            let stream = connection.0;

            if let Err(e) = connect(commands, events, stream) {
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
fn connect(mut commands: Commands, mut events: EventWriter<PlayerCountChange>, stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("New connection: {}", stream.peer_addr().unwrap());

    let client_connect = bincode::deserialize_from::<&TcpStream, LobbyUpdateClient>(&stream)?;
    println!("Client name: {:?}",client_connect);

    if let LobbyUpdateClient::Connect {name} = client_connect {
        // if mp_state.streams.is_empty() {
        //     bincode::serialize_into(&stream, &LobbyUpdateServer::YouAreLobbyLeader)?;
        // }
        // TODO: Assign lobby leader

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

    // TODO: Send current player updates
    //send_current_players_update(mp_state)?;

    Ok(())
}

