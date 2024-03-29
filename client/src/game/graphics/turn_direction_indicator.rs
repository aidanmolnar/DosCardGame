use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy_sprite3d::{Sprite3d, Sprite3dParams};
use dos_shared::{GameInfo, GameState, TurnDirection};
use iyes_loopless::prelude::*;

use super::assets::TurnDirectionIndicatorHandle;

const ROTATION_RATE: f32 = std::f32::consts::TAU * 2. / 60.; // 2 rotations per minute

#[derive(Component)]
struct TurnDirectionIndicator;

pub struct TurnDirectionIndicatorPlugin;

impl Plugin for TurnDirectionIndicatorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_enter_system(GameState::InGame, setup_indicator)
            .add_exit_system(GameState::InGame, cleanup_indicator)
            .add_system(spin_indicator.run_if_resource_exists::<GameInfo>());
    }
}

pub fn setup_indicator(
    mut commands: Commands,
    mut params: Sprite3dParams,
    handle: Res<TurnDirectionIndicatorHandle>,
) {
    commands
        .spawn(
            Sprite3d {
                image: handle.texture.clone(),
                pixels_per_metre: 3.,
                partial_alpha: true,
                unlit: true,
                transform: Transform::from_translation(Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.05,
                }),
                ..default()
            }
            .bundle(&mut params),
        )
        .insert(PickableBundle::default())
        .insert(TurnDirectionIndicator);
}

fn cleanup_indicator(mut commands: Commands, query: Query<Entity, With<TurnDirectionIndicator>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

fn spin_indicator(
    mut query: Query<&mut Transform, With<TurnDirectionIndicator>>,
    game_info: Res<GameInfo>,
    time: Res<Time>,
    mut last_direction: Local<TurnDirection>,
) {
    for mut transform in query.iter_mut() {
        if last_direction.to_owned() != *game_info.current_direction() {
            transform.rotate_x(std::f32::consts::PI);
        }
        match game_info.current_direction() {
            TurnDirection::Clockwise => {
                transform.rotate_z(-ROTATION_RATE * time.delta_seconds());
            }
            TurnDirection::CounterClockwise => {
                transform.rotate_z(ROTATION_RATE * time.delta_seconds());
            }
        }
    }

    *last_direction = game_info.current_direction().clone();
}
