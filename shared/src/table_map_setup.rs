use super::GameState;

use super::table::*;


use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct TableConstructionPlugin;

impl Plugin for TableConstructionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_loopless_state(TableConstructionState::NotStarted)

        // State chain for building tables
        .add_enter_system(
            GameState::InGame, 
            |mut commands: Commands| commands.insert_resource(NextState(TableConstructionState::TableMapCreation))
        )
        .add_enter_system(
            TableConstructionState::TableMapCreation, 
            |mut commands: Commands| commands.insert_resource(NextState(TableConstructionState::TableCreation))
        ).add_enter_system(
            TableConstructionState::TableCreation, 
            |mut commands: Commands| commands.insert_resource(NextState(TableConstructionState::Completed))
        )

        // Clear table map and entities on returning to main menu
        .add_enter_system(
            GameState::MainMenu, 
            remove_tables.run_if_resource_exists::<TableMap>()
        );
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum TableConstructionState {
    NotStarted,
    TableMapCreation,
    TableCreation,
    Completed,
}

// Creates an entity for each location and adds them to a TableMap
pub fn build_table_map(
    mut commands: Commands,
    num_players: usize,
) {
    let mut map = TableMap::default();

    map.0.insert(
        Location::Deck, 
        commands.spawn().id()
    );

    map.0.insert(
        Location::DiscardPile, 
        commands.spawn().id()
    );

    map.0.insert(
        Location::Staging, 
        commands.spawn().id()
    );

    spawn_player_hand_tables(
        &mut map,
        &mut commands,
        num_players,
    );

    commands.insert_resource(map);
}

fn spawn_player_hand_tables(
    map: &mut TableMap,
    commands: &mut Commands,
    num_players: usize,
) {
    for player_id in 0..num_players {
        map.0.insert(
            Location::Hand{player_id}, 
            commands.spawn().id()
        );
    }
}

fn remove_tables(
    table_map: Res<TableMap>,
    mut commands: Commands,
) {
    for (_, entity) in &table_map.0 {
        commands.entity(*entity).despawn();
    }
    commands.remove_resource::<TableMap>();

    commands.insert_resource(NextState(TableConstructionState::NotStarted));
}



