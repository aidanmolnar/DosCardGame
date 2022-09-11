mod networking;
mod ui;

use super::{GameState, MultiplayerState, connections};

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_renet::renet::RenetClient;
use iyes_loopless::prelude::*;

// Ui for main menu and all networking related to lobby
pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(EguiPlugin)
        .init_resource::<ui::UiState>() 
        .add_system_to_stage(CoreStage::PostUpdate, connections::exit_system)

        .add_system(
            ui::lobby_ui_system
            .run_in_state(GameState::MainMenu)
        )
        .add_system_to_stage(CoreStage::PostUpdate,
            networking::lobby_network_system
            .run_if_resource_exists::<RenetClient>()
        );

    }
}