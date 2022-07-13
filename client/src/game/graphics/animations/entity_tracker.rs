
use super::targeting::BoardPosition;
use super::mouse_over::MouseOffset;
use super::updating::AnimationBlueprint;

use super::super::layout::expressions::*;
use super::super::layout::constants::*;

use super::super::assets::*;

const HIGHLIGHT_SCALE: f32 = 1.25;

const WIDTH_OFFSET: f32 = (CARD_WIDTH * HIGHLIGHT_SCALE + CARD_WIDTH) / 2.;
const HIGHLIGHT_Y_OFFSET: f32 = CARD_HEIGHT * (HIGHLIGHT_SCALE - 1.) / 2.;

fn horizontal_offset(hand_size: usize) -> f32 {
    let hand_spacing = your_max_hand_width(hand_size) / (hand_size - 1) as f32; 
    WIDTH_OFFSET - hand_spacing
}

use bevy::prelude::*;

// Tracks the state of entities representing drawn cards
#[derive(Default)]
pub struct AnimationEntityTracker {
    pub discard_pile: Vec<Entity>,
    pub hands: Vec<HandOfCards>,

    pub focused_card: Option<Entity>,
    // TODO: should this be somewhere else?  Why? any function with game manager needs this information...
    pub board_centers: Vec<(f32,f32)>,
    pub player_id: usize,
}   

pub struct HandOfCards (pub Vec<Entity>);

impl AnimationEntityTracker {
    // Adds a card to a players hand
    pub fn deal_card(
        &mut self, 
        player_id: usize, 
        hand_position: Option<usize>, 
        entity: Entity
    ) {
        let hand = self.hands.get_mut(player_id).expect("Invalid owner id");
        if let Some(hand_position) = hand_position {
            hand.0.insert(hand_position, entity);
        } else {
            hand.0.push(entity);
        }
    }

    // Transfers a card from a player's hand to the discard pile
    // TODO: this should return the entity
    pub fn play_card(
        &mut self, 
        player_id: usize, 
        hand_position: usize
    ) -> Entity {
        let hand = self.hands.get_mut(player_id).expect("Invalid owner id");

        if hand_position >= hand.0.len() {
            panic!("Invalid hand position {hand_position} for player {player_id}")
        }

        let entity = hand.0.remove(hand_position);
        self.discard_card(entity);

        entity
    }

    // Adds a card to the discard pile
    // TODO: this should return the entity
    pub fn discard_card(
        &mut self,
        entity: Entity,
    ) {
        self.discard_pile.push(entity);
    }

    // Clears the discard pile for reshuffling
    pub fn clear_discard_pile(
        &mut self
    ) {
        self.discard_pile.clear();
    }
    
    pub fn update_your_targets (
        &self,
        commands: &mut Commands,
    ) {
        let hand = &self.hands.get(self.player_id).unwrap().0;

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

    // TODO: reduce duplicate code: code for adding position / updating animation could be turned into function
    pub fn update_opponent_targets(
        &self,
        commands: &mut Commands,
        player_id: usize
    ) {
        let hand = &self.hands.get(player_id).unwrap().0;

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
        let end = Vec3::new(DISCARD_LOCATION.0,DISCARD_LOCATION.1,0.1 * self.discard_pile.len() as f32);

        commands.entity(entity)
            .insert(BoardPosition {
                position: end 
            }).insert(AnimationBlueprint);
    }

    // TODO: Move focus updating code to here 
    // TODO: Add animation blueprint system
    pub fn update_your_focus(
        &self,
        commands: &mut Commands,
    ) {
        let hand = &self.hands.get(self.player_id).unwrap().0;

        // Find focused card
        if let Some(focused_entity) = self.focused_card {
            let focused_index = hand.iter().position(|&e| e == focused_entity).unwrap();
            let offset = horizontal_offset(hand.len());

            for (i, entity) in hand.iter().enumerate() {

                let offset_sign = (i as isize - focused_index as isize).signum()as f32;

                if i == focused_index {
                    commands.entity(*entity).insert(MouseOffset {
                        offset: HIGHLIGHT_Y_OFFSET * Vec3::Y,
                        scale: HIGHLIGHT_SCALE,
                    }).insert(AnimationBlueprint);
                } else {
                    commands.entity(*entity).insert(MouseOffset {
                        offset: offset * offset_sign * Vec3::X,
                        scale: 1.,
                    }).insert(AnimationBlueprint);
                }
                
            }
        } else {
            for entity in hand.iter() {
                commands.entity(*entity).insert(MouseOffset {
                    offset: Vec3::ZERO,
                    scale: 1.,
                }).insert(AnimationBlueprint);
            }
        }
    }
}

