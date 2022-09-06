use dos_shared::{GameState, messages::lobby::GameSnapshot};

mod ui;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::lobby::lobby_network_system;

pub struct ReconnectPlugin;

impl Plugin for ReconnectPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(
            ui::reconnect_ui
            .run_in_state(GameState::Reconnect))
        .add_system(
            lobby_network_system
            .run_in_state(GameState::Reconnect)
        )
        .add_exit_system(
            GameState::Reconnect, 
            |mut commands: Commands| commands.remove_resource::<GameSnapshot>() 
        );
    }
}
