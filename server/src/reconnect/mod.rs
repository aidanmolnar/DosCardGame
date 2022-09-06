use dos_shared::GameState;

mod connections;
mod networking;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use self::connections::on_leave_system;

pub struct ReconnectPlugin;

impl Plugin for ReconnectPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(
            networking::reconnect_network_system
            .run_in_state(GameState::Reconnect)
        ).add_system(
            connections::playercount_change_system
            .run_in_state(GameState::Reconnect)
        ).add_system(
            connections::handle_connection_task
            .run_in_state(GameState::Reconnect)
        ).add_exit_system(
            GameState::Reconnect, 
            on_leave_system
        );
    }
}


