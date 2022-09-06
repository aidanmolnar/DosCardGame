use super::GameState;
use super::multiplayer;

mod networking;
use networking::{lobby_network_system, leave_lobby_system};
mod connections;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(lobby_network_system
            .run_in_state(GameState::MainMenu)
        )
        .add_system(connections::playercount_change_system
            .run_in_state(GameState::MainMenu)
            
        ).add_system(connections::handle_connection_task
            .run_in_state(GameState::MainMenu)
        )

        .add_exit_system(GameState::MainMenu, leave_lobby_system);
    }
}