

use super::MultiplayerState;
use super::InterfaceManager;
use super::layout::expressions::*;
use super::layout::constants::*;
use super::entity_tracker::HandOfCards;

use bevy::prelude::*;

// TODO: split this up, make it more readable
pub fn setup_interface_manager (
    mp_state: Res<MultiplayerState>,
    mut interface_manager: ResMut<InterfaceManager>,
) {
    interface_manager.player_id = mp_state.turn_id as usize;

    let num_players = mp_state.player_names.len() as u8;
    let num_other_players = num_players - 1;

    for _ in 0..num_players {
        interface_manager.tracker.hands.push(HandOfCards(Vec::new()))
    }

    for owner_id in 0..num_players {

        if owner_id == mp_state.turn_id{
            interface_manager.board_centers.push(YOUR_HAND_CENTER);
        } else {
            // Adjust other ids so your hand is skipped
            let local_id = if owner_id > mp_state.turn_id{
                (owner_id-1) % num_other_players
            } else {
                owner_id % num_other_players
            };
        
            // Arrange centers of opponents hands in an arc
            let (x,y) = arange_arc(
                num_other_players as usize, 
                local_id as usize,
                OPPONENT_ARC_ANGLE);
            let center_x = OPPONENT_ARC_WIDTH*x;
            let center_y = OPPONENT_ARC_HEIGHT*y;

            interface_manager.board_centers.push( (center_x,center_y));
        }
    }
}