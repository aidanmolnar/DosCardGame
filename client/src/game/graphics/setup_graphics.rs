
use super::MultiplayerState;
use super::interface_constants::*;
use super::card_indexing::*;
use super::arange::*;
use super::assets::CardHandles;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_mod_picking::*;

pub struct HandLocations{
    pub centers: Vec<(f32,f32)>,
}

// TODO: split this up, make it more readable
pub fn calculate_hand_locations (
    mut commands: Commands,
    mp_state: Res<MultiplayerState>,
) {
    let num_players = mp_state.player_names.len() as u8;
    let num_other_players = num_players - 1;

    let mut centers = Vec::new();

    for owner_id in 0..num_players {

        if owner_id == mp_state.turn_id{
            centers.push(YOUR_HAND_CENTER);
        } else {
            // Adjust other ids so your hand is skipped
            let local_id = if owner_id > mp_state.turn_id{
                (owner_id-1) % num_other_players
            } else {
                owner_id % num_other_players
            };
        
            // Arrange centers of opponents hands in an arc
            let (x,y) = arange_arc(
                num_other_players as usize, 
                local_id as usize,
                OPPONENT_ARC_ANGLE);
            let center_x = OPPONENT_ARC_WIDTH*x;
            let center_y = OPPONENT_ARC_HEIGHT*y;

            centers.push( (center_x,center_y));
        }
    }

    commands.insert_resource(HandLocations{centers});
}

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