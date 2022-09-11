mod core;
mod targeting;
mod tracker;
mod setup_table;
mod table;

pub use self::{
    core::components, 
    targeting::FocusedCard, 
    table::AnimationItem,
    tracker::{AnimationTracker, DelayedAnimationAction, AnimationAction}, 
};

use dos_shared::{
    GameState, 
    table_map::TableConstructionState
};

use super::{layout, deck, card_indexing};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(core::CoreAnimationPlugin)
        .add_plugin(targeting::TargetingPlugin)
        
        // Create table arrangers and animation tables
        .add_enter_system(
            TableConstructionState::TableCreation, 
            setup_table::add_arrangers
        )
        .add_enter_system(
            TableConstructionState::TableCreation, 
            setup_table::add_animation_tables
        )

        // Create the animation queue when entering the game
        .add_exit_system(GameState::MainMenu, 
            |mut commands: Commands| {
            commands.init_resource::<tracker::AnimationActionQueue>();
        })

        // Execute actions in the animation queue
        .add_system(
            tracker::update_animation_actions
            .run_in_state(GameState::InGame)
        );
    }
}