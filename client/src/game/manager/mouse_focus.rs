
use super::InterfaceManager;
use super::layout::expressions::*;
use super::assets::*;
use super::animations::components::*;

use bevy::prelude::*;

const HIGHLIGHT_SCALE: f32 = 1.25;

const WIDTH_OFFSET: f32 = (CARD_WIDTH * HIGHLIGHT_SCALE + CARD_WIDTH) / 2.;
const HIGHLIGHT_Y_OFFSET: f32 = CARD_HEIGHT * (HIGHLIGHT_SCALE - 1.) / 2.;

fn horizontal_offset(hand_size: usize) -> f32 {
    let hand_spacing = your_max_hand_width(hand_size) / (hand_size - 1) as f32; 
    WIDTH_OFFSET - hand_spacing
}

impl InterfaceManager {

    pub fn set_focused_card(
        &mut self,
        commands: &mut Commands,
        entity: Option<Entity>
    ) {
        self.focused_card = entity;
        self.update_your_focus(commands);
    }


    pub fn update_your_focus(
        &self,
        commands: &mut Commands,
    ) {
        let hand = &self.tracker.hands.get(self.player_id).unwrap().0;

        // TODO: This is horrible, please fix it
        // Find focused card
        if let Some(focused_entity) = self.focused_card {
            if let Some(focused_index) = hand.iter().position(|&e| e == focused_entity) {
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