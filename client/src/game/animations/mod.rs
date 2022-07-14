use super::GameState;
use super::manager::InterfaceManager;

pub mod components;
mod mouse_focus;
mod systems;

use bevy::prelude::*;
use bevy_mod_picking::*;
use iyes_loopless::prelude::*;



pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(
            mouse_focus::focus_system
            .run_on_event::<PickingEvent>()
            .run_in_state(GameState::InGame)
            .before(systems::animation_update_system))
        .add_system(
            systems::animation_update_system
            .run_in_state(GameState::InGame))
        .add_system(
            systems::animation_run_system
            .run_in_state(GameState::InGame)
        );
    }
}