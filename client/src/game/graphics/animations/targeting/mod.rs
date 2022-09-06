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
        // Mouse targeting
        .init_resource::<mouse::FocusedCard>()
        .add_system(mouse::focus_system
            //.run_in_state(GameState::InGame)
            .run_on_event::<PickingEvent>()
        )
        .add_system(mouse::update_system
             //.run_in_state(GameState::InGame)
        )

        // Position targeting
        .add_system(position::update_system
            //.run_in_state(GameState::InGame)
        );
    }
}