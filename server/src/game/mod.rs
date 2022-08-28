use super::GameState;
use super::multiplayer;

mod networking;
mod setup;
mod deal;

mod card_tracker;

use card_tracker::{ServerCardTracker, ServerTable};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app

        // TODO: find some other way to order these systems that makes more sense
        .add_exit_system(GameState::MainMenu, setup::spawn_tables)
        .add_exit_system(GameState::MainMenu, card_tracker::setup_memorized_cards)
        .add_enter_system(GameState::InGame, deal::deal_out_starting_hands)

        .add_system(networking::game_network_system
            .run_in_state(GameState::InGame)
        );
    }
}