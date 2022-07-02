use super::GameState;
use super::card_tracker::CardTracker;
use super::MultiplayerState;

// TODO: keep these private -> see dealing
pub mod animations;
mod arange;
pub mod assets;
pub mod card_indexing;
pub mod interface_constants;
mod setup_graphics;

use animations::{HandUpdated, run_animations, set_card_targets};
use assets::load_assets;
use setup_graphics::{add_deck_sprite, add_camera, calculate_hand_locations};

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy::ecs::event::Events;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Events<HandUpdated>>()

        // Animation systems
        .add_system(run_animations
            .run_in_state(GameState::InGame))
        .add_system(set_card_targets
            .run_in_state(GameState::InGame)
            .run_on_event::<HandUpdated>()
            .before("dealing") // TODO: Clean up this dependency
            )
        
        // On state startup
        .add_enter_system(GameState::InGame, add_deck_sprite)
        .add_enter_system(GameState::InGame, add_camera)
        .add_enter_system(GameState::InGame, calculate_hand_locations)

        // On app startup
        .add_startup_system(load_assets);
    }
}