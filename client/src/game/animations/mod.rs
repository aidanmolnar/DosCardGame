use super::GameState;

pub mod components;
mod systems;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(
            systems::retarget
            .run_in_state(GameState::InGame)
            .label("retarget")
        )

        .add_system(
            systems::run
            .run_in_state(GameState::InGame)
            .after("retarget")
        );
    }
}