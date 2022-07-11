use dos_shared::*;

use super::multiplayer::{Agent,AgentTracker,NetPlayer};
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
            .run_in_state(GameState::MainMenu)
        ).add_system_to_stage(
            CoreStage::PostUpdate,
            handle_playercount_change_system
            .run_in_state(GameState::MainMenu)
            .run_on_event::<PlayerCountChange>()
        );
    }
}

pub struct PlayerCountChange;

fn listen_for_connections(
    listener: Res<TcpListener>, 
    mut commands: Commands, 
    thread_pool: Res<AsyncComputeTaskPool>,
    game_state: Res<CurrentState<GameState>>,
) {
    // accept connections and process them
    match listener.accept() {
        Ok(connection) => {
            let stream = connection.0;

            match game_state.0 {
                GameState::MainMenu => {
                    let task = thread_pool.spawn(async move {
                        create_connection_task(stream)
                    });
                    commands.spawn().insert(task);
                }
                GameState::InGame => {
                    // TODO: clean up these task entities or just don't make them to begin with (i.e. keep single threaded)
                    let task = thread_pool.spawn(async move {
                        create_rejection_task(stream);
                    });
                    commands.spawn().insert(task);
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

    let client_connect = match bincode::deserialize_from::<&TcpStream, LobbyUpdateClient>(&stream) {
        Ok(c) => {c}
        Err(e) => {
            println!("Aborting new connection: {e}");
            // TODO: Shouldn't panic
            stream.shutdown(std::net::Shutdown::Both).expect("Couldn't close stream!");
            return None;
        }
    };
    println!("Client name: {:?}",client_connect);

    if let LobbyUpdateClient::Connect {name} = client_connect {
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

type ConnectionTask = Task<Option<(String, TcpStream)>>;

fn handle_connection_task(
    mut transform_tasks: Query<(Entity, &mut ConnectionTask)>,
    mut commands: Commands,
    mut agent_tracker: ResMut<AgentTracker>,
    mut events: EventWriter<PlayerCountChange>, 
) {
    for (entity, mut task) in transform_tasks.iter_mut() {

        println!("completed the task");

        if let Some(player_option) = future::block_on(future::poll_once(&mut *task)) {
            if let Some((name,stream)) = player_option {
                commands.entity(entity)
                    .remove::<Task<Option<(String, TcpStream)>>>()
                    .insert(NetPlayer { stream})
                    .insert(Agent {name, turn_id: 255});
                
                agent_tracker.agents.push(entity);
                events.send(PlayerCountChange{});
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn disconnect(
    entity: Entity, 
    player: &NetPlayer,
    events: &mut EventWriter<PlayerCountChange>, 
    commands: &mut Commands,
    agent_tracker: &mut ResMut<AgentTracker>
) {
    println!("disconnect ocurred");

    if let Err(e) = bincode::serialize_into(&player.stream, &LobbyUpdateServer::Disconnect) {
        println!("Disconnect message failed to send {e}");
    }

    let index = agent_tracker.agents.iter().position(|e| *e == entity).unwrap();
    agent_tracker.agents.remove(index);

    events.send(PlayerCountChange{});
    commands.entity(entity).despawn();
}

// TODO: Rename this to something less wordy and more descriptive
pub fn handle_playercount_change_system(
    mut query: Query<(&mut Agent, Option<&NetPlayer>)>, 
    mut events: ResMut<Events<PlayerCountChange>>,
    agent_tracker: Res<AgentTracker>,
) {
    if !events.is_empty() {
        println!("Player count changed");
        events.clear();
        // TODO: Agents should have names, not netplayers
        let names = query.iter().map(
            |(agent, _)| agent.name.clone())
        .collect::<Vec<_>>();

        // Update all the players about the current lobby state
        for (i,entity) in agent_tracker.agents.iter().enumerate() {
            let (mut agent, player_option, ) = query.get_mut(*entity).unwrap();

            agent.turn_id = i as u8;

            if let Some(player) = player_option {
                if let Err(e) = bincode::serialize_into(&player.stream, 
                    &LobbyUpdateServer::CurrentPlayers{
                        player_names: names.clone(), 
                        turn_id: i as u8}) 
                {
                    println!("Error sending message to lobby leader {}: {e}", agent.name)
                    // TODO: Should disconnect
                }
            }
        }

    }
}