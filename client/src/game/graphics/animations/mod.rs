use super::GameState;

pub mod entity_tracker;
mod running;
mod targeting;
mod updating;
mod mouse_over;

use bevy::prelude::*;
use bevy_mod_picking::*;
use iyes_loopless::prelude::*;

#[derive(Component)]
pub struct LinearAnimation {
    pub start: Transform,
    pub end: Transform,
    pub timer: Timer,
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(
            mouse_over::focus_system
            .run_on_event::<PickingEvent>()
            .run_in_state(GameState::InGame)
            .before(updating::animation_update_system))
        .add_system(
            updating::animation_update_system
            .run_in_state(GameState::InGame))
        .add_system(
            running::animation_run_system
            .run_in_state(GameState::InGame)
        );
    }
}