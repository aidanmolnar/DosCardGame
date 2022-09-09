use self::connections::exit_system;

use super::GameState;
use super::MultiplayerState;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_renet::renet::RenetClient;
use iyes_loopless::prelude::*;

mod networking;
mod ui;
mod connections;

use connections::handle_connection_task;
pub use networking::lobby_network_system;
use ui::*;
pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(EguiPlugin)
        .init_resource::<UiState>() 
        .add_system_to_stage(CoreStage::PostUpdate, exit_system)

        .add_system(lobby_ui
            .run_in_state(GameState::MainMenu)
        )
        .add_system(handle_connection_task
            .run_in_state(GameState::MainMenu)
        )
        .add_system(lobby_network_system
            .run_if_resource_exists::<RenetClient>()
        );


    }
}