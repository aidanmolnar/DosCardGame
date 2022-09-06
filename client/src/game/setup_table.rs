use dos_shared::{table::*, table_map::{TableConstructionPlugin, TableConstructionState, build_table_map, TableMap}, messages::lobby::GameSnapshot};

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
                dbg!(mp_state.player_names.len());
                build_table_map(commands, mp_state.player_names.len())
            }
        )
        .add_enter_system(
            TableConstructionState::TableCreation, 
            add_client_tables
        );
    }
}

fn add_client_tables(
    mut commands: Commands,
    table_map: Res<TableMap>,
    snapshot_opt: Option<Res<GameSnapshot>>,
) {
    dbg!(snapshot_opt.is_some());
    // Load from server snapshot of state
    if let Some(snapshot) = snapshot_opt {
        for (location, entity) in &table_map.0 {
            let table_snapshot = snapshot.tables[location].clone(); // TODO: Shouldn't need to clone
            let table = ClientTable::from_snapshot(table_snapshot);
            commands.entity(*entity).insert(table);
        }
        
    // Start from initial state
    } else {
        for (location, entity) in &table_map.0 {
            let table = match location {
                Location::Deck => {
                    ClientTable::new_with_size(108)
                },
                Location::DiscardPile => ClientTable::new_empty(),
                Location::Staging => ClientTable::new_empty(),
                Location::Hand { ..} => ClientTable::new_empty(),
            };
    
            commands.entity(*entity).insert(table);
        }
    }
}