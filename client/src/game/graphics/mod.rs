mod animations;
mod layout;
mod assets;
mod camera;
mod card_indexing;
mod deck;
mod remove_cards;

pub use deck::DeckBuilder;
pub use animations::{
    components, 
    FocusedCard, 
    AnimationTracker, 
    DelayedAnimationAction, 
    AnimationAction
};
pub use card_indexing::{SpriteIndex, CARD_BACK_SPRITE_INDEX};
pub use layout::constants;
pub use assets::DosButtonHandle;

use dos_shared::GameState;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(animations::AnimationPlugin)

        .add_startup_system(assets::load_assets)
        .add_startup_system(camera::add_camera)

        // Clear cards when game is over or disconnected
        .add_enter_system(GameState::MainMenu, remove_cards::remove_all_cards);
    }
}