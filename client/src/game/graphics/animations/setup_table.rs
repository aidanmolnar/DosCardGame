use dos_shared::{
    DECK_SIZE, 
    messages::lobby::GameSnapshot, 
    table::Location, 
    table_map::TableMap
};

use crate::game::MultiplayerState;
use super::{
    layout::{
        expressions::arange_arc, 
        constants::{
            DECK_LOCATION, 
            DISCARD_LOCATION, 
            MAX_HAND_WIDTH, 
            MAX_OPPONENT_HAND_WIDTH, 
            OPPONENT_ARC_ANGLE, 
            OPPONENT_ARC_HEIGHT, 
            OPPONENT_ARC_WIDTH, 
            STAGING_LOCATION, 
            YOUR_HAND_CENTER
    }}, 
    table::AnimationTable, 
    deck::DeckBuilder
};

use super::targeting::TableArranger;

use bevy::prelude::*;

// Generates tables for tracking the client animation state
pub fn add_animation_tables(
    mut commands: Commands,
    table_map: Res<TableMap>,
    mp_state: Res<MultiplayerState>,
    mut deck_builder: DeckBuilder,
    snapshot_opt: Option<Res<GameSnapshot>>,
) {
 
    // Load from server snapshot of state (when reconnecting to game)
    if let Some(snapshot) = snapshot_opt {
        for (location, entity) in &table_map.0 {
            let table_snapshot = snapshot.tables[location].clone(); // TODO: Shouldn't need to clone
            let table = match location {
                Location::Hand { player_id } => {
                    if *player_id == mp_state.turn_id {
                        AnimationTable::sorted_from_snapshot(
                            &mut deck_builder, table_snapshot,
                        )
                    } else {
                        AnimationTable::unsorted_from_snapshot(
                            &mut deck_builder, table_snapshot,
                        )
                    }
                }
                _ => 
                AnimationTable::unsorted_from_snapshot(
                    &mut deck_builder, table_snapshot,
                ),
            };
            commands.entity(*entity).insert(table);
        }
    // Start from initial state
    } else {
        for (location, entity) in &table_map.0 {
            let table = match location {
                Location::Deck => {
                    AnimationTable::new_unsorted_with_items(
                        deck_builder.make_unknown_cards(DECK_SIZE)
                    )
                },
                Location::Hand { player_id } => {
                    if *player_id == mp_state.turn_id {
                        AnimationTable::new_sorted()
                    } else {
                        AnimationTable::new_unsorted()
                    }
                }
                Location::DiscardPile | Location::Staging => AnimationTable::new_unsorted(),
            };
    
            commands.entity(*entity).insert(table);
        }
    }
    
}

// Generates arrangers for calculating position of cards
pub fn add_arrangers(
    mut commands: Commands,
    table_map: Res<TableMap>,
    mp_state: Res<MultiplayerState>,
) {
    let num_players = mp_state.player_names.len();

    for (location, entity) in &table_map.0 {
        let arranger = match location {
            Location::Deck => 
                TableArranger{
                    center: DECK_LOCATION,
                    max_width: 0.,
                },
            Location::DiscardPile => 
                TableArranger{
                    center: DISCARD_LOCATION,
                    max_width: 0.,
                },
            Location::Staging => 
                TableArranger{
                    center: STAGING_LOCATION,
                    max_width: 0.,
                },
            Location::Hand { player_id } => {
                if *player_id == mp_state.turn_id {
                    TableArranger{
                        center: YOUR_HAND_CENTER,
                        max_width: MAX_HAND_WIDTH,
                    }
                } else {
                    let local_id = get_local_perspective_id(*player_id, mp_state.turn_id, num_players);
                    let (x,y) = arange_arc(
                        num_players - 1, 
                        local_id ,
                        OPPONENT_ARC_ANGLE);
                    TableArranger{
                        center: (OPPONENT_ARC_WIDTH*x, OPPONENT_ARC_HEIGHT*y),
                        max_width: MAX_OPPONENT_HAND_WIDTH,
                    }
                }
            }
        };

        commands.entity(*entity).insert(arranger);
    }
}

// Reorders ids from players perspective.
// Client is in position 0.  Player to left of client is 1. Next is 2. Player to right of client is num_players - 1.
fn get_local_perspective_id(
    player_id: usize,
    local_id: usize,
    num_players: usize,
) -> usize {
    // Difference between client id and the other player's id
    let difference = isize::try_from(player_id  ).expect("Player ids should not be large enough to wrap") 
                   - isize::try_from(local_id   ).expect("Player ids should not be large enough to wrap");
    let num_players = isize::try_from(num_players).expect("Player count should not be large enough to wrap");

    // Wraps ids between 0 and num_players
    (difference.rem_euclid(num_players) - 1).try_into().expect("Result should be positive")
}