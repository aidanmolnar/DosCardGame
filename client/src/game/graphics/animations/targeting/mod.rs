use super::layout;
use super::core;
use super::table::AnimationTable;

mod mouse;
mod position;

pub use position::TableArranger;
pub use mouse::FocusedCard;

use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;
use iyes_loopless::prelude::*;

pub struct TargetingPlugin;

impl Plugin for TargetingPlugin {
    fn build(&self, app: &mut App) {
        app
        // Getting focused cards
        .init_resource::<mouse::FocusedCard>()
        .add_system(mouse::focus_system
            .run_on_event::<PickingEvent>()
        )

        // Updating card targets
        .add_system(mouse::update_system)
        .add_system(position::update_system);
    }
}