use dos_shared::GameState;

mod animations;
mod layout;
mod assets;
mod camera;
mod card_indexing;
mod deck;

pub use deck::DeckBuilder;
pub use animations::{components, FocusedCard, AnimationTracker, DelayedAnimationAction, AnimationAction};
pub use card_indexing::SpriteIndex;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(animations::AnimationPlugin)

        .add_startup_system(assets::load_assets)
        .add_enter_system(GameState::InGame, camera::add_camera);
    }
}