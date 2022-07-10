use dos_shared::*;
use super::GameState;
use super::multiplayer::{NetPlayer,TurnId};
use super::connection_listening::{PlayerCountChange, disconnect};

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy::ecs::event::Events;

use::bincode;
use std::net::TcpStream;
use std::io;


pub fn leave_lobby_system (
    query: Query<&NetPlayer>
) {
    for player in query.iter() {
        if let Err(e) = bincode::serialize_into(&player.stream, &LobbyUpdateServer::StartGame) {
            println!("Leave lobby message failed to send {e}");
            // TODO: might need to disconnect client here, or return to lobby?
        }
    }
}

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
                commands.insert_resource(NextState(GameState::InGame));
                println!("Should start the game!");
            } else {
                println!("Non-lobby leader sent start game message");
            }
        }
    }
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

// TODO: Break into sub functions
// Rename this to something less wordy and more descriptive
pub fn handle_playercount_change_system(
    mut query: Query<(Entity, &mut NetPlayer, Option< &mut TurnId>)>, 
    mut events: ResMut<Events<PlayerCountChange>>,
    mut commands: Commands) {
    
    if !events.is_empty() {
        println!("Player count change, reassigning ids");
        events.clear();

        // Sort entities by existing id so ids can be reassigned
        let mut entities = query.iter_mut().collect::<Vec<_>>();
        entities.sort_by_key(|e| 
            match &e.2 {
                Some(i) => {i.id}
                None => {255} // Players without an id are added to end
            }
        );

        // Reassign turn ids starting from 0
        for (i,(entity, _, turn_id_opt))in entities.iter_mut().enumerate() {
            match turn_id_opt {
                Some(turn_id) => {turn_id.id = i as u8},
                None => {
                    commands.entity(*entity).insert(TurnId{id:i as u8});
                }
            }
        }

        // Collect player names
        let names = entities.iter().map(|x| x.1.name.clone()).collect::<Vec<_>>();

        // Update all the players about the current lobby state
        for (i,(_,player,_)) in entities.iter().enumerate() {
            if let Err(e) = bincode::serialize_into(&player.stream, 
                &LobbyUpdateServer::CurrentPlayers{
                    player_names: names.clone(), 
                    turn_id: i as u8}) 
            {
                println!("Error sending message to lobby leader {}: {e}", player.name)
                // TODO: Should disconnect
            }
        }
        
    }
}


