
use super::InterfaceManager;
use super::animations::components::*;
use super::layout::expressions::*;
use super::layout::constants::*;

use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

impl InterfaceManager {
    // TODO: reduce duplicate code: code for adding position / updating animation could be turned into function
    pub fn update_opponent_targets(
        &self,
        commands: &mut Commands,
        player_id: usize
    ) {
        let hand = &self.tracker.hands.get(player_id).unwrap().0;

        let max_width = opponent_max_hand_width(hand.len());
        let center = self.board_centers.get(player_id).unwrap();

        for (hand_position, entity) in hand.iter().enumerate() {

            let pos = max_width * arange_1d(hand.len(), hand_position); 
            let end = Vec3::new(center.0 + pos, center.1, 2. + (hand_position as f32) / 10.);

            commands.entity(*entity)
                .insert(BoardPosition {
                    position: end 
                }).insert(AnimationBlueprint);
        }
    }

    pub fn update_discarded_card_target (
        &self,
        commands: &mut Commands,
        entity: Entity,
    ) {
        let end = Vec3::new(DISCARD_LOCATION.0,DISCARD_LOCATION.1,0.1 * self.tracker.discard_pile.len() as f32);

        commands.entity(entity)
            .insert(BoardPosition {
                position: end 
            })
            .insert(AnimationBlueprint)
            .remove::<MouseOffset>()
            .remove_bundle::<PickableBundle>();
    }

    pub fn update_your_targets (
        &self,
        commands: &mut Commands,
    ) {
        let hand = &self.tracker.hands.get(self.player_id).unwrap().0;

        let max_width =  your_max_hand_width(hand.len());
        let center = self.board_centers.get(self.player_id).unwrap();

        for (hand_position, entity) in hand.iter().enumerate() {

            let pos = max_width * arange_1d(hand.len(), hand_position); 
            let end = Vec3::new(center.0 + pos, center.1, 2. + (hand_position as f32) / 10.);

            commands.entity(*entity)
                .insert(BoardPosition {
                    position: end 
                }).insert(AnimationBlueprint);
        }
    }
}