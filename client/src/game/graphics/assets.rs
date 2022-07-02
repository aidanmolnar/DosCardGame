
use bevy::prelude::*;

// Asset path to card sprites
const CARDS_PATH: &str = "UNO_cards.png";

pub struct CardHandles {
    pub atlas: Handle<TextureAtlas>,
    pub mesh: Handle<Mesh>,
}

pub fn load_assets (
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let texture_handle = asset_server.load(CARDS_PATH);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(240.0, 360.0), 13, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mesh_handle = meshes.add(Mesh::from(shape::Quad::default()));
    commands.insert_resource(CardHandles{atlas: texture_atlas_handle, mesh: mesh_handle});
}