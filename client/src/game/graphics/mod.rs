mod assets;
mod animations;
mod layout;
mod camera;
mod card_indexing;
mod deck;
mod remove_cards;
mod background;


use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
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
use bevy_sprite3d::Sprite3dPlugin;
use iyes_loopless::prelude::*;


pub struct GraphicsPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetState {
    NotLoaded,
    Loaded,
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(Sprite3dPlugin)
        
        .add_plugin(animations::AnimationPlugin)
        .add_plugin(background::BackgroundPlugin)

        .add_loopless_state(AssetState::NotLoaded)
        .add_loading_state(
            LoadingState::new(AssetState::NotLoaded)
                .continue_to_state(AssetState::Loaded)
                .with_collection::<assets::CardHandles>()
                .with_collection::<assets::DosButtonHandle>(),
        )
        //.add_startup_system(assets::load_assets)
        .add_startup_system(camera::add_camera)

        // Clear cards when game is over or disconnected
        .add_enter_system(GameState::MainMenu, remove_cards::remove_all_cards);
    }
}