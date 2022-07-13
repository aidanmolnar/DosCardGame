use super::card_building::card_indexing::*;
use super::assets::CardHandles;
use super::layout::constants::*;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_mod_picking::*;

pub fn add_deck_sprite (
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    card_atlas: Res<CardHandles>,
) {
    let texture_atlas_handle = texture_atlases.get_handle(&card_atlas.atlas);
    // Add the deck
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite { index: CARD_BACK_SPRITE_INDEX, ..default() },
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(Vec3::new(DECK_LOCATION.0, DECK_LOCATION.1, 0.)).with_scale(Vec3::splat(1.0)),
        ..default()
    });
}

pub fn add_camera(
    mut commands: Commands,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::FixedVertical;
    camera.orthographic_projection.scale = HEIGHT_SCALE / 2.;

    commands.spawn_bundle(camera).insert_bundle(PickingCameraBundle::default());
}