mod ui;

use dos_shared::{GameState, messages::lobby::GameSnapshot};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

// Ui for lobby when a player disconnected mid game
pub struct ReconnectPlugin;

impl Plugin for ReconnectPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(
            ui::reconnect_ui_system
            .run_in_state(GameState::Reconnect))
        .add_exit_system(
            GameState::Reconnect, 
            |mut commands: Commands| commands.remove_resource::<GameSnapshot>() 
        );
    }
}
