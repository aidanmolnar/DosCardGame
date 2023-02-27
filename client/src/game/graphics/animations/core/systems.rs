use super::components::{BoardPosition, LinearAnimation, MouseOffset};

use bevy::prelude::*;

use std::ops::{Add, Mul, Sub};

const CARD_TRANSFER_TIME: f32 = 0.1;

// Advances linear animations
pub fn run_system(mut query: Query<(&mut LinearAnimation, &mut Transform)>, time: Res<Time>) {
    for (mut target, mut transform) in &mut query {
        target.timer.tick(time.delta());

        transform.translation = lerp(
            target.start.translation,
            target.end.translation,
            target.timer.percent(),
        );

        transform.scale = lerp(target.start.scale, target.end.scale, target.timer.percent());
    }
}

// Checks for changed targets/offsets and updates LinearAnimation to reflect
#[allow(clippy::type_complexity)]
pub fn retarget_system(
    mut query: Query<
        (
            &mut LinearAnimation,
            &Transform,
            &BoardPosition,
            &MouseOffset,
        ),
        Or<(Changed<BoardPosition>, Changed<MouseOffset>)>,
    >,
) {
    for (mut animation, transform, board_position, mouse_offset) in &mut query {
        animation.start = *transform;
        animation.end = Transform::from_translation(board_position.position + mouse_offset.offset)
            .with_scale(Vec3::splat(mouse_offset.scale));

        animation.timer = Timer::from_seconds(CARD_TRANSFER_TIME, TimerMode::Once);
    }
}

// Linear interpolation
fn lerp<T>(start: T, end: T, percent: f32) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T> + Copy,
{
    start + (end - start) * percent
}
