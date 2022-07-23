
use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, HoverEvent};

#[derive(Default)]
pub struct FocusedCard(Option<Entity>);

// Only run on picking event
pub fn focus_system (
    mut focused_card: ResMut<FocusedCard>,
    mut events: EventReader<PickingEvent>,
) {
    let mut option = None;
    let mut update_event = false;

    for event in events.iter() {
        if let PickingEvent::Hover(hover_event) = event {
            update_event = true;

            match hover_event {
                HoverEvent::JustEntered(entity) => {option = Some(*entity)}
                HoverEvent::JustLeft(_) => {}
            }
        }
    }
    
    if update_event {
        focused_card.0 = option;
    }
}

use super::animations::components::*;
use super::table::*;
use super::animations::systems::update_animation;

// TODO: Also need to run this after a card is dealt
// TODO: break this down into smaller functions as well
pub fn update_mouse_system(
    tables: Query<&ClientTable>,
    mut cards: Query<(&mut LinearAnimation, &Transform, &BoardPosition, &mut MouseOffset)>,
    focused_card: Res<FocusedCard>,
) {
    if !focused_card.is_changed() {
        return
    }

    // TODO: skip tables that aren't sorted
    for table in tables.iter() {

        if table.len() == 0 {
            continue;
        }

        let offset = horizontal_offset(table.len());

        let mut offset_sign = if focused_card.0.is_some() {
            -1.
        } else {
            0.
        };

        for entity in table.iter() {
            let query_result  = cards.get_mut(*entity);

            if query_result.is_err() {
                continue;
            }

            let (
                mut animation, 
                transform, 
                board_position,
                mut mouse_offset
            ) = query_result.unwrap();

            if Some(*entity) == focused_card.0 {
                mouse_offset.offset = HIGHLIGHT_Y_OFFSET * Vec3::Y;
                mouse_offset.scale = HIGHLIGHT_SCALE;
                offset_sign = 1.;
            } else {
                mouse_offset.offset = offset * offset_sign * Vec3::X;
                mouse_offset.scale = 1.;
            }
            
            update_animation(
                &mut animation, 
                transform, 
                board_position, 
                Some(&mouse_offset)
            );
            
        }
    }
}

use super::assets::*;
use super::layout::expressions::*;

const HIGHLIGHT_SCALE: f32 = 1.25;

const WIDTH_OFFSET: f32 = (CARD_WIDTH * HIGHLIGHT_SCALE + CARD_WIDTH) / 2.;
const HIGHLIGHT_Y_OFFSET: f32 = CARD_HEIGHT * (HIGHLIGHT_SCALE - 1.) / 2.;

fn horizontal_offset(hand_size: usize) -> f32 {
    let hand_spacing = your_max_hand_width(hand_size) / (hand_size - 1) as f32; 
    WIDTH_OFFSET - hand_spacing
}