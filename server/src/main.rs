use std::net::TcpListener;

use bevy::prelude::*;
use bevy::ecs::event::Events;
use iyes_loopless::prelude::*;

mod lobby_network;
use lobby_network::*;

mod game_network;
use game_network::*;

/// Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    InGame,
}

fn main() {
    // TODO: Maybe move to network file?
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    App::new()
        .add_plugins(MinimalPlugins)

        .init_resource::<MultiplayerState>()
        .insert_resource(listener) // TODO: How to integrate this with iyes?? Deallocate once in game??

        .add_loopless_state(GameState::MainMenu)

        .init_resource::<Events<PlayerCountChange>>()
        .add_exit_system(GameState::MainMenu, leave_lobby_system)
        .add_enter_system(GameState::InGame, enter_game_system)


        // Main menu systems
        .add_system(lobby_network_system
            .run_in_state(GameState::MainMenu)
            .after("handle_changes"))

        .add_system(listen_for_connections
            .run_in_state(GameState::MainMenu)
            .after("handle_changes"))

        .add_system(handle_playercount_change_system
            .run_in_state(GameState::MainMenu)
            .run_on_event::<PlayerCountChange>()
            .label("handle_changes"))
         
        .run()
}

