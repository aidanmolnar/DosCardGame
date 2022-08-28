use dos_shared::table::*;
use dos_shared::cards::*;

use super::ServerTable;

use bevy::prelude::*;

pub fn add_tables(
    mut commands: Commands,
    table_map: Res<TableMap>,
) {
    for (location, entity) in &table_map.0 {
        let table = match location {
            Location::Deck => {
                ServerTable::new(new_deck())
            },
            Location::DiscardPile => ServerTable::default(),
            Location::Hand { .. } => ServerTable::default(),
            Location::Staging => ServerTable::default(),
        };

        commands.entity(*entity).insert(table);
    }
}