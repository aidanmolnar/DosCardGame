use super::GameState;
use super::card_tracker::CardTracker;
use super::MultiplayerState;

pub mod animations;
pub mod spawn_card;
mod arange;
mod assets;
mod card_indexing;
mod interface_constants;
mod setup_graphics;

use animations::{HandUpdated, AnimationPlugin};
use assets::load_assets;
use setup_graphics::{add_deck_sprite, add_camera, calculate_hand_locations};
use spawn_card::SpawnCardSystems;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy::ecs::event::Events;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Events<HandUpdated>>()

        .add_plugin(SpawnCardSystems)
        .add_plugin(AnimationPlugin)
        
        // On state startup
        .add_enter_system(GameState::InGame, add_deck_sprite)
        .add_enter_system(GameState::InGame, add_camera)
        .add_enter_system(GameState::InGame, calculate_hand_locations)

        // On app startup
        .add_startup_system(load_assets);
    }
}