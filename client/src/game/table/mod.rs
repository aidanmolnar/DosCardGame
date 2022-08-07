use super::GameState;

mod transfer;
mod client_table;
mod targeting;
mod setup;

pub use client_table::ClientTable;
pub use targeting::mouse::FocusedCard;
pub use transfer::CardTransferer;

use targeting::{mouse, position};
use targeting::position::TableArranger;

use bevy::prelude::{Plugin, App};
use iyes_loopless::prelude::*;
use bevy_mod_picking::PickingEvent;

pub struct TablePlugin;

impl Plugin for TablePlugin {
    fn build(&self, app: &mut App) {
        app

        // Setup tables
        .add_enter_system(GameState::InGame, setup::spawn_all_tables)

        // Mouse targeting
        .init_resource::<mouse::FocusedCard>()
        .add_system(mouse::focus_system
            .run_in_state(GameState::InGame)
            .run_on_event::<PickingEvent>())
         .add_system(mouse::update_system
             .run_in_state(GameState::InGame))

        // Position targeting
        .add_system(position::update_system
            .run_in_state(GameState::InGame));
        
    }
}