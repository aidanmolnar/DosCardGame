
use super::GameState;
use super::multiplayer;

mod networking;
mod setup;
mod table;
mod game_info;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app

        // TODO: find some other way to order these systems that makes more sense
        .add_exit_system(GameState::MainMenu, setup::spawn_tables)
        .add_enter_system(GameState::InGame, setup::deal_cards)

        .add_system(networking::game_network_system
            .run_in_state(GameState::InGame)
        );
    }
}