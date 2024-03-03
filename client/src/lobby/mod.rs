mod user_api;
mod login_manager;
mod networking;
mod ui;
mod onboarding_ui;

use crate::game::AssetState;

use super::{GameState, MultiplayerState, connections};

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_renet::renet::RenetClient;
use iyes_loopless::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OnboardingState {
    NotOnboarded,
    Authenticated
}

// Ui for main menu and all networking related to lobby
pub struct LobbyPlugin;


impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(EguiPlugin)
        .init_resource::<ui::UiState>() 
        .init_resource::<onboarding_ui::UserChoice>() 
        .init_resource::<user_api::UserDto>() 
        .init_resource::<login_manager::UserCreate>() 
        .add_loopless_state(OnboardingState::NotOnboarded)
        .add_system_to_stage(CoreStage::PostUpdate, connections::exit_system)

        .add_system(
            onboarding_ui::onboarding_ui_system
            .run_in_state(OnboardingState::NotOnboarded)
            .run_in_state(AssetState::Loaded)
        )
        .add_system(
            ui::lobby_ui_system
            .run_in_state(OnboardingState::Authenticated)
            .run_in_state(GameState::MainMenu)
        )
        .add_system_to_stage(CoreStage::PostUpdate,
            networking::lobby_network_system
            .run_if_resource_exists::<RenetClient>()
        );

    }
}
