use super::GameState;
use super::MultiplayerState;

pub mod assets;
pub mod layout;
pub mod card_building;
pub mod animations;
pub mod manager;
pub mod client_actions;
mod setup;
mod server_actions;
mod networking;

pub use manager::InterfaceManager;
use networking::YourTurn;
use networking::game_network_system;
use assets::load_assets;
use setup::{add_deck_sprite, add_camera};
use client_actions::play_card_system;
use server_actions::delayed_dealing_system;


use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_mod_picking::{PickingPlugin,InteractablePickingPlugin, PickingEvent};



pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app

        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)

        .add_plugin(card_building::CardBuildingPlugin)
        .add_plugin(animations::AnimationPlugin)

        // On state startup
        .add_enter_system(GameState::InGame, add_deck_sprite)
        .add_enter_system(GameState::InGame, add_camera)
        .add_enter_system(GameState::InGame, manager::setup_interface_manager)
        
        .init_resource::<InterfaceManager>()

        // On app startup
        .add_startup_system(load_assets)

        .add_system(delayed_dealing_system
            .run_in_state(GameState::InGame))

        .add_system(game_network_system
            .run_in_state(GameState::InGame))

        .add_system(
            play_card_system
            .run_on_event::<PickingEvent>()
            .run_if_resource_exists::<YourTurn>()
        );
    }

}