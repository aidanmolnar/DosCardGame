use super::LinearAnimation;

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