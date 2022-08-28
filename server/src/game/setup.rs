use dos_shared::table::*;
use dos_shared::cards::*;
use dos_shared::GameInfo;

use super::multiplayer::AgentTracker;
use super::ServerTable;

use bevy::prelude::*;


pub fn spawn_tables (
    mut commands: Commands,
    agent_tracker: Res<AgentTracker>,
) {
    let mut map = TableMap::default();

    let starting_deck = new_deck(); //deck_builder.make_cards(105);

    // Make deck table
    let table = ServerTable::new(starting_deck);
    let deck_entity = commands.spawn()
        .insert(table).id();
    map.0.insert(Location::Deck, deck_entity);

    // Make discard table
    let table = commands.spawn()
        .insert(ServerTable::default()).id();
    map.0.insert(Location::DiscardPile, table);

    // Make staging table
    let table = commands.spawn()
        .insert(ServerTable::default()).id();
    map.0.insert(Location::Staging, table);

    spawn_player_hand_tables(
        &mut map,
        &mut commands,
        agent_tracker.agents.len()
    );

    commands.insert_resource(GameInfo::new(agent_tracker.agents.len()));
    commands.insert_resource(map);
}

fn spawn_player_hand_tables(
    map: &mut TableMap,
    commands: &mut Commands,
    num_players: usize,
) {
    for player_id in 0..num_players {
        let table = commands.spawn()
            .insert(ServerTable::default()).id();
        map.0.insert(Location::Hand{player_id}, table);
    }
}