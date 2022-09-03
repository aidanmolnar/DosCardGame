use dos_shared::*;

use super::multiplayer::AgentTracker;
use super::GameState;

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::ecs::event::Events;
use iyes_loopless::prelude::*;
use bevy::tasks::Task;

use std::net::{TcpListener, TcpStream};
use std::io;
use futures_lite::future;

pub struct ConnectionListeningPlugin;

impl Plugin for ConnectionListeningPlugin {
    fn build(&self, app: &mut App) {
        let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
        listener.set_nonblocking(true).expect("Cannot set non-blocking on listener!");

        app
        .init_resource::<AgentTracker>()
        .init_resource::<Events<PlayerCountChange>>()
        .insert_resource(listener) // TODO: How to integrate this with iyes?? Deallocate once in game??
        
        .add_system_to_stage(
            CoreStage::Update,
            listen_for_connections
        ).add_system_to_stage(
            CoreStage::Update,
            handle_connection_task
            //.run_in_state(GameState::MainMenu)
        ).add_system_to_stage(
            CoreStage::PostUpdate,
            handle_playercount_change_system
            //.run_in_state(GameState::MainMenu)
            .run_on_event::<PlayerCountChange>()
        );
    }
}

pub enum PlayerCountChange{
    Connect,
    Disconnect(usize),
}

// TODO: break up / simplify this function
fn listen_for_connections(
    listener: Res<TcpListener>, 
    mut commands: Commands, 
    game_state: Res<CurrentState<GameState>>,
) {
    let thread_pool = AsyncComputeTaskPool::get();
    // accept connections and process them
    match listener.accept() {
        Ok(connection) => {
            let stream = connection.0;

            match game_state.0 {
                GameState::MainMenu => {
                    let task = thread_pool.spawn(async move {
                        create_connection_task(stream)
                    });
                    commands.spawn().insert(ConnectionTask(task));
                }
                GameState::InGame | GameState::PostGame => {
                    // TODO: clean up these task entities or just don't make them to begin with (i.e. keep single threaded)
                    let task = thread_pool.spawn(async move {
                        create_rejection_task(stream);
                    });
                    commands.spawn().insert(RejectionTask(task));
                }
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

fn create_connection_task(stream: TcpStream) -> Option<(String, TcpStream)> {
    println!("New connection: {}", stream.peer_addr().unwrap());

    let client_connect = match bincode::deserialize_from::<&TcpStream, messages::lobby::FromClient>(&stream) {
        Ok(c) => {c}
        Err(e) => {
            println!("Aborting new connection: {e}");
            // TODO: Shouldn't panic
            stream.shutdown(std::net::Shutdown::Both).expect("Couldn't close stream!");
            return None;
        }
    };
    println!("Client name: {:?}",client_connect);

    if let messages::lobby::FromClient::Connect {name} = client_connect {
        Some((name, stream))
    } else {
        None
    }
}


fn create_rejection_task(stream: TcpStream) {
    println!("Rejecting a connection");
    // TODO: Send a rejection message
    // TODO: Shouldn't panic
    stream.shutdown(std::net::Shutdown::Both).expect("Couldn't close rejected stream!");
}

#[derive(Component)]
struct RejectionTask(Task<()>);


#[derive(Component)]
struct ConnectionTask(Task<Option<(String, TcpStream)>>);
 
fn handle_connection_task(
    mut transform_tasks: Query<(Entity, &mut ConnectionTask)>,
    mut commands: Commands,
    mut agent_tracker: ResMut<AgentTracker>,
    mut events: EventWriter<PlayerCountChange>, 
) {
    for (entity, mut task) in transform_tasks.iter_mut() {

        println!("completed the task");

        if let Some(player_option) = future::block_on(future::poll_once(&mut task.0)) {
            if let Some((name,stream)) = player_option {
                agent_tracker.new_player(name, stream);
                events.send(PlayerCountChange::Connect);
            } 

            commands.entity(entity).despawn();
        }
    }
}

// TODO: Is there a way to handle this by spawning an event or resource or entity 
//       so that all of these resources don't need to be passed around
pub fn disconnect(
    player: usize,
    events: &mut EventWriter<PlayerCountChange>, 
) {
    events.send(PlayerCountChange::Disconnect(player));
}

// TODO: Rename this to something less wordy and more descriptive
pub fn handle_playercount_change_system(
    mut events: ResMut<Events<PlayerCountChange>>,
    mut agent_tracker: ResMut<AgentTracker>,
) {
    if !events.is_empty() {
        println!("Player count changed");

        remove_disconnected_players(&mut events, &mut agent_tracker);
        
        // TODO: Agents should have names, not netplayers
        let names = agent_tracker.names();

        // Update all the players about the current lobby state
        for (player, stream) in agent_tracker.iter_ids_and_streams() {

            if let Err(e) = bincode::serialize_into(stream, 
                &messages::lobby::FromServer::CurrentPlayers{
                    player_names: names.clone(), 
                    turn_id: player as u8}) 
            {
                panic!("Error sending message to lobby leader {player}: {e}")
                // TODO: Should disconnect not panic
            }
            
        }

    }
}

fn remove_disconnected_players(
    events: &mut ResMut<Events<PlayerCountChange>>,
    agent_tracker: &mut ResMut<AgentTracker>,
) {
    let mut to_remove = events.drain().filter_map(|event| {
        match event {
            PlayerCountChange::Connect => None,
            PlayerCountChange::Disconnect(player) => Some(player),
        }
    }).collect::<Vec<_>>();

    to_remove.sort();
    to_remove.reverse();
    for r in to_remove {
        agent_tracker.remove(r);
    }

}