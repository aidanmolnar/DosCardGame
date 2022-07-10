use super::GameState;
use super::MultiplayerState;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use iyes_loopless::prelude::*;

mod networking;
mod ui;
mod connecting;

use connecting::handle_connection_task;
use networking::lobby_network_system;
use ui::*;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(EguiPlugin)
        .init_resource::<UiState>()
        .init_resource::<MultiplayerState>() // This should be moved to a more generic location
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(lobby_ui)
                .with_system(handle_connection_task)
                .with_system(lobby_network_system)
                .into()
        );
    }
}