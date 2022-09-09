use bevy::app::AppExit;
use bevy::ecs::system::SystemParam;
use bevy_renet::renet::ServerEvent;
use dos_shared::*;
use dos_shared::channel_config::LOBBY_CHANNEL_ID;
use dos_shared::channel_config::connection_config;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_renet::renet::RenetServer;
use bevy_renet::renet::ServerAuthentication;
use bevy_renet::renet::ServerConfig;
use dos_shared::messages::lobby::FromServer;

use std::marker::PhantomData;
use std::net::UdpSocket;
use std::time::SystemTime;

use crate::game::ServerGame;
use crate::multiplayer::MultiplayerState;

pub struct ConnectionListeningPlugin;

impl Plugin for ConnectionListeningPlugin {
    fn build(&self, app: &mut App) {
        let server = new_renet_server();

        app
        .insert_resource(server)
        .add_system(connection_events_system
            .run_on_event::<ServerEvent>())
        .add_system(reconnections_system
            .run_in_state(GameState::Reconnect)
            .run_on_event::<ServerEvent>()
            .after(connection_events_system)
        )
        .add_system(exit_system);
    }
}

// TODO: should take command line input instead of using constant
fn new_renet_server() -> RenetServer {
    let server_addr = DEFAULT_IP.parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = connection_config();
    let server_config = ServerConfig::new(99, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

#[derive(SystemParam)]
struct ConnectionManager<'w, 's> {
    pub renet_server: ResMut<'w, RenetServer>,
    pub mp_state: ResMut<'w, MultiplayerState>,
    pub game_state: Res<'w,CurrentState<GameState>>,

    #[system_param(ignore)]
    _phantom: PhantomData<&'s()>,
}

fn connection_events_system(
    mut manager: ConnectionManager,
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
) {
    let mut player_count_change = false;

    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, user_data) => {
                let connected = manager.handle_connect_event(
                    *id, 
                    user_data
                );
                player_count_change = player_count_change || connected;
            }
            ServerEvent::ClientDisconnected(id) => {
                manager.mp_state.disconnect_player(*id);
                player_count_change = true;
                println!("Client {} disconnected", id);
            }
        }
    }

    // Decide whether to remove disconnected or not
    match &manager.game_state.0 {
        GameState::MainMenu | GameState::PostGame => {
            manager.mp_state.remove_disconnected_players();
        },
        GameState::InGame | GameState::Reconnect => {},
    }

    // Enter reconnect state if in game and player count updated
    if manager.game_state.0 == GameState::InGame && player_count_change {
        commands.insert_resource(NextState(GameState::Reconnect));
    }

    // Send an update message to players if the lobby has changed
    if player_count_change {
        // Players enter reconnect state if they receive a lobby update while in game state
        manager.send_player_count_update()
    }

    // Check if every player has disconnected and return to the main menu if so
    if manager.mp_state.all_disconnected() {
        println!("All players disconnected, returning to main menu.");
        commands.insert_resource(NextState(GameState::MainMenu));
    }
}

impl ConnectionManager<'_,'_> {
    // Returns true if the player was connected
    fn handle_connect_event(
        &mut self, 
        renet_id: u64,
        user_data: &[u8; 256],
    ) -> bool {
        
        // Try and parse the player's name
        // Reject the connection if the name can't be converted
        let name = 
        if let Ok(raw_str) = String::from_utf8(user_data.to_vec()) {
            raw_str.trim_matches(char::from(0)).to_string()
        } else {
            self.send_reject_message(
                renet_id, 
                "Invalid name.".to_owned()
            );
            return false;
        };
        
        println!("Client {} ({}) connected", renet_id, name);  

        let mut player_count_updated = false;
    
        // Decide what to do with connection based on game state
        match self.game_state.0 {
            GameState::MainMenu | GameState::PostGame => {
                self.mp_state.new_player(name, renet_id);
                player_count_updated = true;
            },
            GameState::InGame => {
                self.send_reject_message(
                    renet_id, 
                    "Cannot join once game has started.".to_owned()
                );
            },
            GameState::Reconnect => {
                // Only allow players that were disconnected to rejoin, otherwise reject
                // Recognizes player based just on their username
                if let Some(player) = self.mp_state.player_from_name(&name) {
                    if self.mp_state.is_disconnected(player) {
                        player_count_updated = true;
                        self.mp_state.reconnect_player(player, renet_id);
                    } else {
                        self.send_reject_message(
                            renet_id, 
                            "Invalid name.".to_owned()
                        );
                    }
                } else {
                    self.send_reject_message(
                        renet_id, 
                        "Only previously connected players can rejoin.".to_owned()
                    );
                }
            },
        }

        player_count_updated
    }


    fn send_reject_message(
        &mut self, 
        renet_id: u64,
        reason: String
    ) {
        let message = bincode::serialize(
            &FromServer::Reject { reason}
        ).expect("Failed to serialize message");
    
        self.renet_server.send_message(renet_id, LOBBY_CHANNEL_ID, message)
    }

    fn send_player_count_update(
        &mut self
    ) {
        let names = self.mp_state.names();

        for (turn_id, renet_id) in self.mp_state.iter_players() {
            let message = bincode::serialize(
                &FromServer::CurrentPlayers{
                    player_names: names.clone(), 
                    turn_id: turn_id as u8
                }
            ).expect("Failed to serialize message");

            self.renet_server.send_message(renet_id, LOBBY_CHANNEL_ID, message)
        }
    }
}



// Runs after handle_events_system only in reconnect state (needed because ServerGame doesn't exist in main menu)
fn reconnections_system(
    mut renet_server: ResMut<RenetServer>,
    mut mp_state: ResMut<MultiplayerState>,
    mut commands: Commands,
    game: ServerGame,
) {
    // Send game state snapshot to recently reconnected players
    for (player, renet_id) in mp_state.send_state_to_desynced_players() {
        let message = bincode::serialize(
            &FromServer::Reconnect(game.get_snapshot(player))
        ).expect("Failed to serialize message");

        renet_server.send_message(renet_id, LOBBY_CHANNEL_ID, message)
    }

    // Decide to resume game if in reconnect state and all players reconnected
    if mp_state.all_ready() {

        // Re-enter game state
        commands.insert_resource(NextState(GameState::InGame));

        let start_message = bincode::serialize(
            &FromServer::StartGame
        ).expect("Failed to serialize message");

        // Send start messages to all clients
        for (_, renet_id) in mp_state.iter_players() {
            renet_server.send_message(renet_id, LOBBY_CHANNEL_ID, start_message.clone())
        }
    }
}

pub fn exit_system(
    mut renet_server: ResMut<RenetServer>,
    events: EventReader<AppExit>,
) {
    if !events.is_empty() {
        renet_server.disconnect_clients();
    }
}
