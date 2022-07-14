use dos_shared::cards::Card;

use super::entity_tracker::EntityTracker;
use super::local_player_state::LocalPlayerState;
use super::templates;

use bevy::prelude::*;


// Tracks the state of entities representing drawn cards
#[derive(Default)]
pub struct InterfaceManager {
    pub tracker: EntityTracker,
    local_player: LocalPlayerState,

    pub focused_card: Option<Entity>,
    pub board_centers: Vec<(f32,f32)>,
    pub player_id: usize,
}   

impl InterfaceManager {
    pub fn deal_to_you(
        &mut self,
        commands: &mut Commands,
        card: Card,
    ) {
        let hand_position = self.local_player.receive_card(card);
        
        // Spawn new card with MouseOverOffset, Pickable, SpriteSheet,
        // PrimaryTarget, LinearAnimator added later
        let entity = templates::deal_to_you(commands, card);

        self.tracker.deal_card(self.player_id, Some(hand_position), entity);
        
        // Recompute relevent card target positions
        self.update_your_focus(commands);
        self.update_your_targets(commands);
    }

    pub fn deal_to_opponent(
        &mut self,
        commands: &mut Commands,
        player_id: usize,
    ) {
        // Spawn new card with SpriteSheet
        // PrimaryTarget, LinearAnimator added later
        let entity = templates::deal_to_opponent(commands);

        self.tracker.deal_card(player_id, None, entity);

        // Recompute relevent card target positions
        self.update_opponent_targets(commands, player_id)
    }

    pub fn deal_to_discard_pile(
        &mut self,
        commands: &mut Commands,
        card: Card,
    ) {
        // Spawn new card with SpriteSheet
        // PrimaryTarget, LinearAnimator added later
        let entity = templates::deal_to_discard(commands, card);

        self.tracker.discard_card(entity);

        // Recompute relevent card target positions
        self.update_discarded_card_target(commands, entity);
    }

    pub fn you_play_card(
        &mut self,
        commands: &mut Commands,
        card: Card,
    ) {
        let hand_position = self.local_player.play_card(card);
        let entity = self.tracker.play_card(self.player_id, hand_position);

        // Remove the MouseOverOffset, Pickable components
        templates::you_play_card(commands, entity);

        // Recompute relevent card target positions
        self.update_your_focus(commands);
        self.update_your_targets(commands);
        self.update_discarded_card_target(commands, entity);
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
        templates::opponent_play_card(commands, entity, card);

        // Recompute relevent card target positions
        self.update_opponent_targets(commands, player_id);
        self.update_discarded_card_target(commands, entity);
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

    
    
    
}

