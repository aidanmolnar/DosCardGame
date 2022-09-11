pub mod components;
mod systems;

use bevy::prelude::*;

// Linearly interpolates cards to their target destination
pub struct CoreAnimationPlugin;

impl Plugin for CoreAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(
            systems::retarget_system
            .label("retarget")
        )

        .add_system(
            systems::run_system
            .after("retarget")
        );
    }
}