use dos_shared::DECK_SIZE;
use dos_shared::messages::lobby::GameSnapshot;
use dos_shared::table::Location;
use dos_shared::table_map::TableMap;

use crate::game::MultiplayerState;
use super::layout::expressions::*;
use super::layout::constants::*;
use super::table::AnimationTable;
use super::deck::DeckBuilder;

use super::targeting::TableArranger;

use bevy::prelude::*;

// TODO: Make more readable
pub fn add_animation_tables(
    mut commands: Commands,
    table_map: Res<TableMap>,
    mp_state: Res<MultiplayerState>,
    mut deck_builder: DeckBuilder,
    snapshot_opt: Option<Res<GameSnapshot>>,
) {
    dbg!(snapshot_opt.is_some());
    
    // Load from server snapshot of state
    if let Some(snapshot) = snapshot_opt {
        for (location, entity) in &table_map.0 {
            let table_snapshot = snapshot.tables[location].clone(); // TODO: Shouldn't need to clone
            let table = match location {
                Location::Deck => 
                    AnimationTable::unsorted_from_snapshot(
                        &mut deck_builder, table_snapshot,
                    ),
                Location::DiscardPile => 
                    AnimationTable::unsorted_from_snapshot(
                        &mut deck_builder, table_snapshot,
                    ),
                Location::Staging => 
                    AnimationTable::unsorted_from_snapshot(
                        &mut deck_builder, table_snapshot,
                    ),
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
                Location::DiscardPile => AnimationTable::new_unsorted(),
                Location::Staging => AnimationTable::new_unsorted(),
                Location::Hand { player_id } => {
                    if *player_id == mp_state.turn_id {
                        AnimationTable::new_sorted()
                    } else {
                        AnimationTable::new_unsorted()
                    }
                }
            };
    
            commands.entity(*entity).insert(table);
        }
    }
    
}

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
                    let local_id = ((*player_id as isize - mp_state.turn_id as isize).rem_euclid(num_players as isize) - 1) as usize;
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