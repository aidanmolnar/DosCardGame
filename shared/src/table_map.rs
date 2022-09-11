use super::GameState;
use super::table::Location;

use bevy::prelude::*;
use bevy::utils::HashMap;
use iyes_loopless::prelude::*;

// A map from a location (ex. deck, discard pile, a players hand) to the entity that stores Table 
#[derive(Default)]
pub struct TableMap (pub HashMap<Location, Entity>);

// Adds the table map and entities for each table at every location.
// Also automatically advances TableConstructionState
// Cleans up table map and table entities when game is back at main menu
pub struct TableConstructionPlugin;

impl Plugin for TableConstructionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_loopless_state(TableConstructionState::NotStarted)

        // State chain for building tables
        .add_exit_system(
            GameState::MainMenu, 
            |mut commands: Commands| commands.insert_resource(NextState(TableConstructionState::TableMapCreation))
        )
        .add_enter_system(
            TableConstructionState::TableMapCreation, 
            |mut commands: Commands| commands.insert_resource(NextState(TableConstructionState::TableCreation))
        ).add_enter_system(
            TableConstructionState::TableCreation, 
            |mut commands: Commands| commands.insert_resource(NextState(TableConstructionState::Completed))
        )

        // Clear table map and entities when game/round ends
        .add_enter_system(
            GameState::MainMenu, 
            remove_tables.run_if_resource_exists::<TableMap>()
        );
    }
}

// Progressivly build table map at start of the game. Need table map before tables can be added
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



