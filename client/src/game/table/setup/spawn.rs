use dos_shared::table::*;

use crate::game::layout::expressions::*;
use crate::game::layout::constants::*;
use crate::game::MultiplayerState;
use super::client_table::*;
use super::deck::DeckBuilder;
use super::TableArranger;

use bevy::prelude::*;



// TODO: clean this up
pub fn spawn_all_tables (
    mut commands: Commands,
    mp_state: Res<MultiplayerState>,
    mut deck_builder: DeckBuilder,
) {
    let mut map = TableMap::default();

    let starting_deck = deck_builder.make_cards(105);

    // Make deck table
    let table = ClientTable::UnsortedTable(UnsortedTable::new(starting_deck));
    let deck_entity = commands.spawn()
    .insert(
    TableArranger{
        center: DECK_LOCATION,
        max_width: 0.,
    })
    .insert(table).id();
    map.0.insert(Location::Deck, deck_entity);

    // Make discard table
    let discard_pile_entity = spawn_table(
        &mut commands, 
        DISCARD_LOCATION, 
        0., 
        false,
    );
    map.0.insert(Location::DiscardPile, discard_pile_entity);

    spawn_player_hand_tables(
        &mut map,
        &mut commands,
        mp_state.player_names.len(),
        mp_state.turn_id,
    );

    commands.insert_resource(map)
}

fn spawn_player_hand_tables(
    map: &mut TableMap,
    commands: &mut Commands,
    num_players: usize,
    local_player_id: usize,
) {
    //let num_players = mp_state.player_names.len();
    let num_other_players = num_players - 1;

    for player_id in 0..num_players {
        let entity = if player_id == local_player_id {
            spawn_table(
                commands, 
                YOUR_HAND_CENTER, 
                MAX_HAND_WIDTH, 
                true,
            )
        } else {

            // TODO: this is jenky
            // Adjust other ids so your hand is skipped
            let local_id = if player_id > local_player_id {
                (player_id-1) % num_other_players
            } else {
                player_id % num_other_players
            };
        
            // Arrange centers of opponents hands in an arc
            let (x,y) = arange_arc(
                num_other_players, 
                local_id,
                OPPONENT_ARC_ANGLE);
            let center = (OPPONENT_ARC_WIDTH*x, OPPONENT_ARC_HEIGHT*y);

            spawn_table(
                commands, 
                center, 
                MAX_OPPONENT_HAND_WIDTH, 
                false
            )
        };

        map.0.insert(Location::Hand{player_id}, entity);
    }
}

fn spawn_table (
    commands: &mut Commands,
    center: (f32, f32),
    max_width: f32,
    sorted: bool
) -> Entity {
    let table = if sorted {
        ClientTable::SortedTable(SortedTable::default())
    } else {
        ClientTable::UnsortedTable(UnsortedTable::default())
    };

    commands.spawn()
    .insert(
    TableArranger{
        center,
        max_width,
    })
    .insert(table)
    .id()
}