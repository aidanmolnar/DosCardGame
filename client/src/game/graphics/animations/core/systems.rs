use super::components::*;

use bevy::prelude::*;

use std::ops::{Add,Sub,Mul};


// System for animating cards to their target locations
pub fn run (
    mut query: Query<(&mut LinearAnimation, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut target, mut transform) in &mut query {
        target.timer.tick(time.delta());

        transform.translation = lerp(
            target.start.translation, 
            target.end.translation, 
            target.timer.percent()
        );

        transform.scale = lerp(
            target.start.scale, 
            target.end.scale, 
            target.timer.percent()
        );
    }
}  

// System for updating animation target locations
#[allow(clippy::type_complexity)] // This is more readable than defining a new type
pub fn retarget(
    mut query: 
    Query<
        (&mut LinearAnimation, &Transform, &BoardPosition, &MouseOffset),
        Or<(Changed<BoardPosition>, Changed<MouseOffset>)>, 
    >,
) {
    for (mut animation, transform, board_position, mouse_offset) in &mut query {
        animation.start = *transform;
        animation.end = Transform::from_translation(board_position.position + mouse_offset.offset).with_scale(Vec3::splat(mouse_offset.scale));
        animation.timer = Timer::from_seconds(0.1, false);
    }
}

// Linear interpolation
fn lerp<T>(start: T, end: T, percent: f32) -> T 
where 
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<f32, Output = T>,
    T: Copy,
{
    start + (end - start) * percent
}