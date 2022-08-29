use dos_shared::table::*;

use crate::game::layout::expressions::*;
use crate::game::layout::constants::*;
use crate::game::MultiplayerState;
use crate::game::table::animation_table::AnimationTable;
use crate::game::table::card_tracker::ClientTable;
use super::deck::DeckBuilder;
use super::TableArranger;

use bevy::prelude::*;

pub fn add_animation_tables(
    mut commands: Commands,
    table_map: Res<TableMap>,
    mp_state: Res<MultiplayerState>,
    mut deck_builder: DeckBuilder,
) {
    for (location, entity) in &table_map.0 {
        let table = match location {
            Location::Deck => {
                AnimationTable::new_unsorted_with_items(deck_builder.make_cards(108))
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