use dos_shared::table::*;

use crate::game::animations::components::*;
use crate::game::layout::{expressions::*, constants::*};
use super::ClientTable;


use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, HoverEvent};

#[derive(Default)]
pub struct FocusedCard(Option<CardReference>);

// Only run on picking event
pub fn focus_system (
    mut focused_card: ResMut<FocusedCard>,
    mut events: EventReader<PickingEvent>,
    map: Res<TableMap>,
    tables: Query<&ClientTable>, 
) {
    let mut focus_entity_option = None;
    let mut update_event = false;

    for event in events.iter() {
        if let PickingEvent::Hover(hover_event) = event {
            update_event = true;

            match hover_event {
                HoverEvent::JustEntered(entity) => {focus_entity_option = Some(*entity)}
                HoverEvent::JustLeft(_) => {}
            }
        }
    }
    
    if update_event {
        if let Some(focus_entity) = focus_entity_option {
            focused_card.0 = locate_card (
                &map,
                &tables,
                focus_entity
            );
        } else {
            focused_card.0 = None;
        }
    }
}

// Converts a card entity into a card reference (location and hand position)
// Brute force, could be sped up with some sort of hashing scheme but require a lot of transactional logic/time whenever a card is moved
fn locate_card (
    map: &Res<TableMap>,
    tables: &Query<&ClientTable>,
    entity: Entity
) -> Option<CardReference> {
    for (location, table_entity) in &map.0 {
        let table 
            = tables.get(*table_entity).unwrap();
        
        for (hand_position, card_entity) in table.iter().enumerate() {
            if *card_entity == entity {
                return Some(CardReference{location: *location, index: Some(hand_position)})
            }
        }
    }

    None
}

pub fn update_system (
    tables: Query<&ClientTable>,
    changed_tables: Query<&ClientTable, Changed<ClientTable>>,
    mut mouse_offsets: Query<&mut MouseOffset>,
    focused_card: Res<FocusedCard>,
) {
    // Only run the system if card focus has changed or a table has been changed
    if !focused_card.is_changed() && changed_tables.is_empty() {
        return
    }

    // Skip tables that aren't sorted
    for table in tables.iter()
    .filter(|x| matches!(x, ClientTable::SortedTable(_))) {

        if let Some(card_reference) = &focused_card.0{
            // Find the index of the focused card in the table or reset
            if let Some(focused_index) = card_reference.index {
                // If focused card is in table
                calculate_offsets(
                    &mut mouse_offsets, 
                    table.iter(), 
                    table.len(), 
                    focused_index
                );
            } else {
                // If focused card is not in table
                reset_offsets(
                    &mut mouse_offsets, 
                    table.iter()
                );
            }
        } else {
            // If there is no focused card
            reset_offsets(
                &mut mouse_offsets, 
                table.iter()
            );
        }

    }
}

fn reset_offsets <'a> (
    mouse_offsets: &mut Query<&mut MouseOffset>,
    entities: impl Iterator<Item = &'a Entity>,
) {
    for entity in entities {
        let mut mouse_offset 
            = mouse_offsets.get_mut(*entity).unwrap();
        mouse_offset.offset = Vec3::ZERO;
        mouse_offset.scale = 1.;
    }
}

// Calculate appropriate offset for each card
fn calculate_offsets <'a> (
    mouse_offsets: &mut Query<&mut MouseOffset>,
    entities: impl Iterator<Item = &'a Entity>,
    num_cards: usize,
    focused_index: usize,
) {
    let offset = horizontal_offset(num_cards);

    for (i, entity) in entities.enumerate() {
        let mut mouse_offset  
            = mouse_offsets.get_mut(*entity).unwrap();
        
        if i == focused_index {
            mouse_offset.offset = HIGHLIGHT_Y_OFFSET * Vec3::Y;
            mouse_offset.scale = HIGHLIGHT_SCALE;
        } else {
            mouse_offset.offset = offset * (i as isize - focused_index as isize).signum() as f32 * Vec3::X;
            mouse_offset.scale = 1.;
        }
    }
}