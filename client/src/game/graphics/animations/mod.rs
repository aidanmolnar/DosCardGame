mod core;
mod setup_table;
mod table;
mod targeting;
mod tracker;

pub use self::{
    core::components,
    table::AnimationItem,
    targeting::FocusedCard,
    tracker::{AnimationAction, AnimationTracker, DelayedAnimationAction},
};

use dos_shared::{table_map::TableConstructionState, GameState};

use super::{card_indexing, deck, layout};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(core::CoreAnimationPlugin)
            .add_plugin(targeting::TargetingPlugin)
            // Create table arrangers and animation tables
            .add_enter_system(
                TableConstructionState::TableCreation,
                setup_table::add_arrangers,
            )
            .add_enter_system(
                TableConstructionState::TableCreation,
                setup_table::add_animation_tables,
            )
            // Create the animation queue when entering the game
            .add_exit_system(GameState::MainMenu, |mut commands: Commands| {
                commands.init_resource::<tracker::AnimationActionQueue>();
            })
            // Execute actions in the animation queue
            .add_system(tracker::update_animation_actions.run_in_state(GameState::InGame));
    }
}
