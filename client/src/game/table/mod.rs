use memorized_cards::MemorizedCards;
use setup::ClientTableSetupPlugin;

use self::animation_tracker::AnimationActionQueue;

use super::GameState;

mod animation_table;
mod card_tracker;
mod targeting;
mod setup;
mod animation_tracker;
mod memorized_cards;

pub use targeting::mouse::{FocusedCard,FocusedCardData};
pub use setup::DeckBuilder;
pub use card_tracker::ClientCardTracker;

use targeting::{mouse, position};
use targeting::position::TableArranger;

use bevy::prelude::{Plugin, App};
use iyes_loopless::prelude::*;
use bevy_mod_picking::PickingEvent;

pub struct TablePlugin;

impl Plugin for TablePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<MemorizedCards>()

        // Setup tables
        .add_plugin(ClientTableSetupPlugin)

        // Setup card tracker
        //.add_enter_system(GameState::InGame, delayed_transfers::setup_delayed_transfer_queue)

        // Mouse targeting
        .init_resource::<mouse::FocusedCard>()
        .add_system(mouse::focus_system
            .run_in_state(GameState::InGame)
            .run_on_event::<PickingEvent>())
         .add_system(mouse::update_system
             .run_in_state(GameState::InGame))

        // Position targeting
        .add_system(position::update_system
            .run_in_state(GameState::InGame))
        
        // Update delayed transfers in card tracker
        .init_resource::<AnimationActionQueue>()
        .add_system(animation_tracker::update_animation_actions
            .run_in_state(GameState::InGame));
    }
}