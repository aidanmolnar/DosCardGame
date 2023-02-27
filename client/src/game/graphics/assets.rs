use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use iyes_loopless::prelude::*;

// Card dimensions (pixels)
pub const CARD_WIDTH: f32 = 240.;
pub const CARD_HEIGHT: f32 = 360.;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetState {
    NotLoaded,
    Loaded,
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(AssetState::NotLoaded)
            .add_loading_state(
                LoadingState::new(AssetState::NotLoaded)
                    .continue_to_state(AssetState::Loaded)
                    .with_collection::<CardHandles>()
                    .with_collection::<DosButtonHandle>()
                    .with_collection::<TurnDirectionIndicatorHandle>(),
            );
    }
}

// Handles to card resources
#[derive(AssetCollection, Resource)]
pub struct CardHandles {
    #[asset(texture_atlas(tile_size_x = 240., tile_size_y = 360., columns = 15, rows = 5))]
    #[asset(path = "UNO_cards.png")]
    pub atlas: Handle<TextureAtlas>,
    #[asset(path = "glow.png")]
    pub glow: Handle<Image>,
}

// Handles to the "call dos" button resources
#[derive(AssetCollection, Resource)]
pub struct DosButtonHandle {
    #[asset(path = "call_dos_button.png")]
    pub texture: Handle<Image>,
}

// Handles to the turn direction indicator resources
#[derive(AssetCollection, Resource)]
pub struct TurnDirectionIndicatorHandle {
    #[asset(path = "cycle.png")]
    pub texture: Handle<Image>,
}
