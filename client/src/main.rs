mod lobby_network;
use lobby_network::*;

mod game_network;
use game_network::*;

mod mainmenu_ui;
use mainmenu_ui::*;

mod graphics;
use graphics::*;

//use bevy::app::AppExit;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_egui::EguiPlugin;

const DEFAULT_IP: &str = "localhost:3333";

/// Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
}

// https://github.com/IyesGames/iyes_loopless/blob/main/examples/menu.rs
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<UiState>()
        .init_resource::<MultiplayerState>()

        .add_startup_system(load_assets)
        .insert_resource(WindowDescriptor {
            title: "Dos!".to_string(),
            width: 1920.,
            height: 1080.,
            resizable: true,
            ..default()
        })

        .add_loopless_state(GameState::MainMenu)

        // Main menu systems
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(lobby_ui)
                .with_system(lobby_network_system
                    .run_if_resource_exists::<MultiplayerState>())
                .into()
        )

        // In Game systems
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(game_network_system)
                .into()
        )

        .add_enter_system(GameState::InGame,show_cards_test)
       
        
        .run()
}

