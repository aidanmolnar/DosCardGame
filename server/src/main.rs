use std::net::TcpListener;

use bevy::prelude::*;
use bevy::ecs::event::Events;
use iyes_loopless::prelude::*;

mod lobby_network;
use lobby_network::*;

/// Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    //InGame,
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

         // Main menu systems
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(listen_for_connections) // This might run all the time to allow players to reconnect
                .into()
        )

        .add_system(lobby_network_system.label("main_network"))
        .add_system(handle_playercount_change_system.run_on_event::<PlayerCountChange>().before("main_network"))
        
        .run()
}

