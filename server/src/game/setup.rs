use dos_shared::table::Location;
use dos_shared::cards::new_deck;
use dos_shared::table_map::TableMap;

use super::table::ServerTable;

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
            _ => ServerTable::default(),
        };

        commands.entity(*entity).insert(table);
    }
}