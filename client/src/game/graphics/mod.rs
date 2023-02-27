mod animations;
mod assets;
mod background;
mod camera;
mod card_indexing;
mod deck;
mod layout;
mod remove_cards;
mod turn_direction_indicator;

pub use animations::{
    components, AnimationAction, AnimationTracker, DelayedAnimationAction, FocusedCard,
};
pub use assets::{AssetState, DosButtonHandle};
pub use card_indexing::{SpriteIndex, CARD_BACK_SPRITE_INDEX};
pub use deck::DeckBuilder;
pub use layout::constants;

use dos_shared::GameState;

use bevy::prelude::*;

use bevy_sprite3d::Sprite3dPlugin;
use iyes_loopless::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(assets::AssetPlugin)
            .add_plugin(Sprite3dPlugin)
            .add_plugin(animations::AnimationPlugin)
            .add_plugin(background::BackgroundPlugin)
            .add_plugin(turn_direction_indicator::TurnDirectionIndicatorPlugin)
            .add_startup_system(camera::add_camera)
            // Clear cards when game is over or disconnected
            .add_enter_system(GameState::MainMenu, remove_cards::remove_all_cards);
    }
}
