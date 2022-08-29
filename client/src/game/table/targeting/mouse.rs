use dos_shared::table::*;
use dos_shared::cards::Card;
use dos_shared::transfer::Table;

use crate::game::animations::components::*;
use crate::game::layout::{expressions::*, constants::*};
use super::AnimationTable;


use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, HoverEvent};

#[derive(Default)]
// pub struct FocusedCard (
//     pub Option<(Location, TableIndexData)>,
// );
pub struct FocusedCard (pub Option<FocusedCardData>);

pub struct FocusedCardData {
    pub location: Location,
    pub hand_index: usize,
    pub sorted_index: Option<usize>,
    pub card_value: Option<Card>,
}


// Only run on picking event
pub fn focus_system (
    mut focused_card: ResMut<FocusedCard>,
    mut events: EventReader<PickingEvent>,
    map: Res<TableMap>,
    tables: Query<&AnimationTable>, 
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
           focused_card.0 = 
            locate_card (
                &map,
                &tables,
                focus_entity
            );
        } else {
            focused_card.0 = None;
        }
    }
}



// Brute force, could be sped up with some sort of hashing scheme but require a lot of transactional logic/time whenever a card is moved
// TODO: Add a Location component to each card so we don't have to check all the cards in every table
fn locate_card (
    map: &Res<TableMap>,
    tables: &Query<&AnimationTable>,
    entity: Entity
) -> Option<FocusedCardData> {
    for (location, table_entity) in &map.0 {
        let table 
            = tables.get(*table_entity).unwrap();
        
        if let Some(hand_index) = table.actual_index(entity) {

            // TODO: clean this up, kind of hacky right now
            let card_value = table.card(entity);
            let sorted_index = table.sorted_index(entity);
            return Some(FocusedCardData {
                location: *location,
                hand_index,
                sorted_index,
                card_value,
            })
        }
    }
    None
}

pub fn update_system (
    tables: Query<&AnimationTable>,
    changed_tables: Query<&AnimationTable, Changed<AnimationTable>>,
    mut mouse_offsets: Query<&mut MouseOffset>,
    focused_card: Res<FocusedCard>,
) {
    // Only run the system if card focus has changed or a table has been changed
    if !focused_card.is_changed() && changed_tables.is_empty() {
        return
    }

    // Skip tables that aren't sorted
    for table in tables.iter()
    .filter(|x| matches!(x, AnimationTable::Sorted(_))) {

        if let Some(focused_card_data) = &focused_card.0 {
            // Find the index of the focused card in the table or reset
            if let Some(sorted_index) = focused_card_data.sorted_index {
                calculate_offsets(
                    &mut mouse_offsets, 
                    table.iter_entities(), 
                    table.len(), 
                    sorted_index,
                );
            } else {
                reset_offsets(
                    &mut mouse_offsets, 
                    table.iter_entities()
                );
            }
        } else {
            // If there is no focused card
            reset_offsets(
                &mut mouse_offsets, 
                table.iter_entities()
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