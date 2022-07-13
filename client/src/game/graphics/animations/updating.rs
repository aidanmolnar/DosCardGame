
use bevy::prelude::*;

use super::targeting::BoardPosition;
use super::LinearAnimation;
use super::mouse_over::MouseOffset;

#[derive(Component)]
pub struct AnimationBlueprint;

pub fn animation_update_system (
    query: Query<(Entity, &Transform, &BoardPosition, Option<&MouseOffset>), With<AnimationBlueprint>>,
    mut commands: Commands,
) {
    for (entity,transform, board_position, mouse_option) in query.iter() {
        if let Some(mouse) = mouse_option {
            commands.entity(entity).insert(
                LinearAnimation {
                    start: *transform,
                    end: Transform::from_translation(board_position.position + mouse.offset).with_scale(Vec3::splat(mouse.scale)),
                    timer: Timer::from_seconds(0.1, false),
                }
            ).remove::<AnimationBlueprint>();
        } else {
            commands.entity(entity).insert(
                LinearAnimation {
                    start: *transform,
                    end: Transform::from_translation(board_position.position),
                    timer: Timer::from_seconds(0.1, false),
                }
            ).remove::<AnimationBlueprint>();
        }
    }
}