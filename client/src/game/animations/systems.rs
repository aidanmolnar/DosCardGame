use super::components::*;

use bevy::prelude::*;


// System for animating cards to their target locations
pub fn animation_run_system (
    mut query: Query<(&mut LinearAnimation, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut target, mut transform) in query.iter_mut() {
        target.timer.tick(time.delta());
        // LERP towards target end location
        transform.translation = target.start.translation + (target.end.translation - target.start.translation) * target.timer.percent();
        transform.scale = target.start.scale + (target.end.scale - target.start.scale) * target.timer.percent();
    }
}  

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