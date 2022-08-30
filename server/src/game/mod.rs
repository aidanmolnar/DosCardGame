use dos_shared::GameInfo;
use dos_shared::table_map_setup::*;

use crate::multiplayer::AgentTracker;
use super::GameState;
use super::multiplayer;

mod sync;
mod networking;
mod setup;
mod deal;
mod server_game;
mod table;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        // Create resource for controlling turn advancement
        .add_exit_system(
            GameState::MainMenu, 
            |mut commands: Commands, agent_tracker: Res<AgentTracker>|{
                commands.insert_resource(GameInfo::new(agent_tracker.num_agents()))
            }
        )

        // Create resource for caching card values
        .add_exit_system(GameState::MainMenu, sync::setup_syncer)

        // Setup table map and tables, then deal out starting cards.  Plugin advances state automatically
        .add_plugin(TableConstructionPlugin)
        .add_enter_system(
            TableConstructionState::TableMapCreation, 
            |commands: Commands, agent_tracker: Res<AgentTracker>|{
                build_table_map(commands, agent_tracker.num_agents())
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
        );
    }

}