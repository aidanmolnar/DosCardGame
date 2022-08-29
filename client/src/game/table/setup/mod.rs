use dos_shared::{table_map_setup::*, GameState, GameInfo};

use crate::multiplayer::MultiplayerState;
use super::TableArranger;

mod deck;
mod spawn;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub use deck::DeckBuilder;

pub struct ClientTableSetupPlugin;

impl Plugin for ClientTableSetupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app

        // Create resource for controlling turn advancement
        .add_exit_system(
            GameState::MainMenu, 
            |mut commands: Commands, mp_state: Res<MultiplayerState>|{
                commands.insert_resource(GameInfo::new(mp_state.player_names.len()))
            }
        )

        // Setup table map and tables.  Plugin advances state automatically
        .add_plugin(TableConstructionPlugin)
        .add_enter_system(
            TableConstructionState::TableMapCreation, 
            |commands: Commands, mp_state: Res<MultiplayerState>|{
                build_table_map(commands, mp_state.player_names.len())
            }
        )
        .add_enter_system(
            TableConstructionState::TableCreation, 
            spawn::add_animation_tables
        ).add_enter_system(
            TableConstructionState::TableCreation, 
            spawn::add_client_tables
        ).add_enter_system(
            TableConstructionState::TableCreation, 
            spawn::add_arrangers
        );
    }
}