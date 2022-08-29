use dos_shared::{table::*, table_map_setup::{TableConstructionPlugin, TableConstructionState, build_table_map}};

use crate::multiplayer::MultiplayerState;

use super::table::ClientTable;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct ClientTableSetupPlugin;

impl Plugin for ClientTableSetupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_plugin(TableConstructionPlugin)
        .add_enter_system(
            TableConstructionState::TableMapCreation, 
            |commands: Commands, mp_state: Res<MultiplayerState>|{
                build_table_map(commands, mp_state.player_names.len())
            }
        )
        .add_enter_system(
            TableConstructionState::TableCreation, 
            add_client_tables
        );
    }
}

pub fn add_client_tables(
    mut commands: Commands,
    table_map: Res<TableMap>,
) {
    for (location, entity) in &table_map.0 {
        let table = match location {
            Location::Deck => {
                ClientTable::new_deck(108)
            },
            Location::DiscardPile => ClientTable::new(),
            Location::Staging => ClientTable::new(),
            Location::Hand { ..} => ClientTable::new(),
        };

        commands.entity(*entity).insert(table);
    }
}

