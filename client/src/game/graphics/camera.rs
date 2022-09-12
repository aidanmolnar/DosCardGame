use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;

pub fn add_camera(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 2410.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert_bundle(PickingCameraBundle::default());
}