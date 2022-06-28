mod lobby_network;
use lobby_network::*;
mod mainmenu_ui;
use mainmenu_ui::*;

use bevy::app::AppExit;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_egui::EguiPlugin;

const DEFAULT_IP: &str = "localhost:3333";

// https://github.com/IyesGames/iyes_loopless/blob/main/examples/menu.rs
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<UiState>()
        .init_resource::<MultiplayerState>()

        .add_loopless_state(GameState::MainMenu)

        // Main menu systems
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(lobby_ui)
                .with_system(lobby_network_system
                    .run_if_resource_exists::<MultiplayerState>())
                //
                .into()
        )

        // Stage and systems for ensuring AppExit event is captured
        .add_stage_after(
            CoreStage::Last,
            "very_last",
            SystemStage::single_threaded()
        )
        .add_system_to_stage("very_last", close_event_listener
            .run_on_event::<AppExit>()
            .run_if_resource_exists::<MultiplayerState>()

        )
        
        .run()
}

/// Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    //InGame,
}



fn close_event_listener(mp_state: Res<MultiplayerState>) {
    println!("App Exit Event");

    if let Some(stream) = &mp_state.stream {
        end_connection(stream);
    }
    
}

