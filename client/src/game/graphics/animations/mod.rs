use dos_shared::GameState;
use dos_shared::table_map_setup::TableConstructionState;

use super::layout;
use super::deck;
use super::card_indexing;

mod core;
mod targeting;
mod tracker;
mod setup_table;
mod table;

pub use self::core::components;
pub use self::targeting::FocusedCard;
pub use self::tracker::{AnimationTracker, DelayedAnimationAction, AnimationAction};

use bevy::prelude::*;
use iyes_loopless::prelude::*;


pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(targeting::TargetingPlugin)
        .add_plugin(core::CoreAnimationPlugin)

        // Create table arrangers and animation tables
        .add_enter_system(
            TableConstructionState::TableCreation, 
            setup_table::add_arrangers
        )
        .add_enter_system(
            TableConstructionState::TableCreation, 
            setup_table::add_animation_tables
        )

        .add_enter_system(GameState::InGame, 
            |mut commands: Commands| {
            commands.init_resource::<tracker::AnimationActionQueue>()
        })

        // Update delayed transfers in card tracker
        .add_system(tracker::update_animation_actions
            .run_in_state(GameState::InGame));
    }
}