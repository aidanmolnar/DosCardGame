use super::GameState;
use super::MultiplayerState;
use crate::connections;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_renet::renet::RenetClient;
use iyes_loopless::prelude::*;

mod networking;
mod ui;

pub use networking::lobby_network_system;
use ui::{UiState, lobby_ui};

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(EguiPlugin)
        .init_resource::<UiState>() 
        .add_system_to_stage(CoreStage::PostUpdate, connections::exit_system)

        .add_system(lobby_ui
            .run_in_state(GameState::MainMenu)
        )
        .add_system(lobby_network_system
            .run_if_resource_exists::<RenetClient>()
        );


    }
}