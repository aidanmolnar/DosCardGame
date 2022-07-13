use dos_shared::cards::Card;

use super::MultiplayerState;

use super::graphics::layout::constants::*;
use super::graphics::animations::entity_tracker::{AnimationEntityTracker, HandOfCards};

mod local_player_state;

mod entity_manipulation;

use local_player_state::LocalPlayerState;

use bevy::prelude::*;

// TODO: does this name make sense?
#[derive(Default)]
pub struct GameManager {
    local_player: LocalPlayerState,
    pub tracker: AnimationEntityTracker,
}

// Can we set targets here?
impl GameManager {
    pub fn deal_to_you(
        &mut self,
        commands: &mut Commands,
        card: Card,
    ) {
        let hand_position = self.local_player.receive_card(card);
        
        // Spawn new card with MouseOverOffset, Pickable, SpriteSheet,
        // PrimaryTarget, LinearAnimator added later
        let entity = entity_manipulation::deal_to_you (
            commands,
            card
        );

        self.tracker.deal_card(self.tracker.player_id, Some(hand_position), entity);
        
        // Recompute relevent card target positions
        self.tracker.update_your_focus(commands);
        self.tracker.update_your_targets(commands);
    }

    pub fn deal_to_opponent(
        &mut self,
        commands: &mut Commands,
        player_id: usize,
    ) {
        // Spawn new card with SpriteSheet
        // PrimaryTarget, LinearAnimator added later
        let entity = entity_manipulation::deal_to_opponent(commands);

        self.tracker.deal_card(player_id, None, entity);

        // Recompute relevent card target positions
        self.tracker.update_opponent_targets(commands, player_id)
    }

    pub fn deal_to_discard_pile(
        &mut self,
        commands: &mut Commands,
        card: Card,
    ) {
        // Spawn new card with SpriteSheet
        // PrimaryTarget, LinearAnimator added later
        let entity = entity_manipulation::deal_to_discard(commands, card);

        self.tracker.discard_card(entity);

        // Recompute relevent card target positions
        self.tracker.update_discarded_card_target(commands, entity);
    }

    pub fn you_play_card(
        &mut self,
        commands: &mut Commands,
        card: Card,
    ) {
        let hand_position = self.local_player.play_card(card);
        let entity = self.tracker.play_card(self.tracker.player_id, hand_position);

        // Remove the MouseOverOffset, Pickable components
        entity_manipulation::you_play_card(commands, entity);

        // Recompute relevent card target positions
        self.tracker.update_your_targets(commands);
        self.tracker.update_your_focus(commands);
        self.tracker.update_discarded_card_target(commands, entity);
    }

    pub fn opponent_play_card(
        &mut self,
        commands: &mut Commands,
        player_id: usize,
        hand_position: usize,
        card: Card,
    ) {
        let entity = self.tracker.play_card(player_id, hand_position);

        // Change sprite sheet index
        entity_manipulation::opponent_play_card(commands, entity, card);

        // Recompute relevent card target positions
        self.tracker.update_opponent_targets(commands, player_id);
        self.tracker.update_discarded_card_target(commands, entity);
    }

    // TODO: finish this function
    pub fn reshuffle_deck(
        &mut self,
        commands: &mut Commands,
    ) {
        // Change the sprite sheet index to blanks
        // Animate them back to deck one at a time?

        // tracker.clear_discard_pile()
        // Would need to get discard pile from tracker?  Maybe this should return and then clear?

        //board_state.clear_discard_pile()
    }

    //TODO: rename.  Should these functions be somewhere else?
}



use super::graphics::layout::expressions::arange_arc;

// TODO: split this up, make it more readable
pub fn setup_game_manager (
    mp_state: Res<MultiplayerState>,
    mut game_manager: ResMut<GameManager>,
) {
    game_manager.tracker.player_id = mp_state.turn_id as usize;

    let num_players = mp_state.player_names.len() as u8;
    let num_other_players = num_players - 1;

    for _ in 0..num_players {
        game_manager.tracker.hands.push(HandOfCards(Vec::new()))
    }

    for owner_id in 0..num_players {

        if owner_id == mp_state.turn_id{
            game_manager.tracker.board_centers.push(YOUR_HAND_CENTER);
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

            game_manager.tracker.board_centers.push( (center_x,center_y));
        }
    }
}

