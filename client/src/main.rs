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
use bevy::ecs::event::Events;
use iyes_loopless::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::*;

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
        .insert_resource(WindowDescriptor {
            title: "Dos!".to_string(),
            width: 1920.,
            height: 1080.,
            resizable: true,
            position: Some(Vec2::ZERO),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)

        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)

        .init_resource::<UiState>()
        .init_resource::<MultiplayerState>()

        .add_startup_system(load_assets)

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
                .label("main")
                .run_in_state(GameState::InGame)
                .with_system(game_network_system)
                .with_system(move_targets)
                .with_system(delayed_dealing_system)
                .into()
        )

        .init_resource::<Events<CardChanged>>()

        .add_system(set_card_targets
            .run_in_state(GameState::InGame)
            .run_on_event::<CardChanged>().
            before("main"))

        .add_enter_system(GameState::InGame,add_camera)
        .add_enter_system(GameState::InGame,setup_graphics)
        //.add_enter_system(GameState::InGame,add_deck)
        .add_system_to_stage(CoreStage::PostUpdate, print_events.run_in_state(GameState::InGame))
        
        .run()
}

pub fn print_events(mut events: EventReader<PickingEvent>) {

    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => info!("Gee Willikers, it's a click! {:?}", e),
        }
    }
}