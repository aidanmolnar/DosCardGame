use super::layout::constants::HEIGHT_SCALE;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_mod_picking::*;

pub fn add_camera(
    mut commands: Commands,
) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(HEIGHT_SCALE);

    commands.spawn_bundle(camera).insert_bundle(PickingCameraBundle::default());
}