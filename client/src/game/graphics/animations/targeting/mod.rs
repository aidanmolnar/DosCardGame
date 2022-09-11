mod mouse;
mod position;

pub use position::TableArranger;
pub use mouse::FocusedCard;

use dos_shared::GameState;

use super::{layout, core, table::AnimationTable};

use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;
use iyes_loopless::prelude::*;

// Tracks focused card and sets animation targets and offsets
pub struct TargetingPlugin;

impl Plugin for TargetingPlugin {
    fn build(&self, app: &mut App) {
        app
        // Updating focused card resource on mouseovers
        .init_resource::<mouse::FocusedCard>()
        .add_system(mouse::focus_system
            .run_on_event::<PickingEvent>()
            .run_in_state(GameState::InGame)
        )

        // Updating card targets
        .add_system(mouse::update_system)
        .add_system(position::update_system);
    }
}