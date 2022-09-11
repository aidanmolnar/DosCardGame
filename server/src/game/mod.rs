mod sync;
mod networking;
mod setup;
mod deal;
mod server_game;
mod table;
mod call_dos;

pub use server_game::ServerGame;

use dos_shared::{
    GameInfo, 
    table_map::{
        TableConstructionPlugin, 
        TableConstructionState, 
        build_table_map
    }
};

use crate::multiplayer::MultiplayerState;
use super::GameState;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

// Adds game resources and systems
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        // Create resource for controlling turn advancement
        .add_exit_system(
            GameState::MainMenu, 
            |mut commands: Commands, mp_state: Res<MultiplayerState>|{
                commands.insert_resource(GameInfo::new(mp_state.num_agents()));
            }
        )

        // Create resource for caching card values
        .add_exit_system(
            GameState::MainMenu, 
            sync::setup_syncer
        )

        // Setup table map and tables, then deal out starting cards.  Plugin advances state automatically
        .add_plugin(TableConstructionPlugin)
        .add_enter_system(
            TableConstructionState::TableMapCreation, 
            |commands: Commands, mp_state: Res<MultiplayerState>|{
                build_table_map(commands, mp_state.num_agents());
            }
        )
        .add_enter_system(
            TableConstructionState::TableCreation, 
            setup::add_tables
        )
        .add_enter_system(
            TableConstructionState::Completed, 
            deal::deal_out_starting_hands
        )

        // Handle receiving network events and advancing game state
        .add_system(networking::game_network_system
            .run_in_state(GameState::InGame)
        )

        // "Call dos" systems
        //  TODO: make plugin
        .add_enter_system(
            GameState::MainMenu, 
            |mut commands: Commands| {commands.remove_resource::<call_dos::CallDos>()}
        )
        .add_system(call_dos::call_dos_graceperiod_system
            .run_in_state(GameState::InGame)
            .run_if_resource_exists::<call_dos::CallDos>()
        );
    }

}