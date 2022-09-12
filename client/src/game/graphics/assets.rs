use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;

// Card dimensions (pixels)
pub const CARD_WIDTH: f32 = 240.;
pub const CARD_HEIGHT: f32 = 360.;

// Handles to card resources
#[derive(AssetCollection)]
pub struct CardHandles {
    #[asset(texture_atlas(tile_size_x = 240., tile_size_y = 360., columns = 15, rows = 5))]
    #[asset(path = "UNO_cards.png")]
    pub atlas: Handle<TextureAtlas>,
}

// Handles to the "call dos" button resources
#[derive(AssetCollection)]
pub struct DosButtonHandle {
    #[asset(path = "call_dos_button.png")]
    pub texture: Handle<Image>,
}