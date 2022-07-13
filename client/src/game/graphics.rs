use super::GameState;

pub mod assets;
pub mod layout;
pub mod card_building;
pub mod animations;
mod setup;

use assets::load_assets;
use setup::{add_deck_sprite, add_camera};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_plugin(card_building::CardBuildingPlugin)
        .add_plugin(animations::AnimationPlugin)

        // On state startup
        .add_enter_system(GameState::InGame, add_deck_sprite)
        .add_enter_system(GameState::InGame, add_camera)

        // On app startup
        .add_startup_system(load_assets);
    }
}