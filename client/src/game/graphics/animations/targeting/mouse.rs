use dos_shared::{
    table::{Location,Table}, 
    cards::Card, 
    table_map::TableMap
};

use super::{
    core::components::MouseOffset, 
    AnimationTable, 
    layout::{
        expressions::horizontal_offset, 
        constants::{
            HIGHLIGHT_SCALE, 
            HIGHLIGHT_Y_OFFSET
        }
}};

use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, HoverEvent};

// Resource that keeps track of which card the player has moused over
// None if no card is moused over
#[derive(Default)]
pub struct FocusedCard (pub Option<FocusedCardData>);

pub struct FocusedCardData {
    pub location: Location, // Table card is in
    pub hand_index: usize, // Position based on insertion order
    pub sorted_index: Option<usize>, // None if not a sorted table
    pub card_value: Option<Card>, // None if face down
}

// Updates focused card resource based on mouse overs
pub fn focus_system (
    mut focused_card: ResMut<FocusedCard>,
    mut events: EventReader<PickingEvent>,
    map: Res<TableMap>,
    tables: Query<&AnimationTable>, 
) {
    let mut focus_entity_option = None; // Potential hovered entity
    let mut hover_event_occured = false;

    // Loop over mouse hover events events
    for event in events.iter() {
        if let PickingEvent::Hover(hover_event) = event {
            hover_event_occured = true;

            match hover_event {
                HoverEvent::JustEntered(entity) => {focus_entity_option = Some(*entity)}
                HoverEvent::JustLeft(_) => {}
            }
        }
    }
    
    if hover_event_occured {
        if let Some(focus_entity) = focus_entity_option {
            // Get focused card data for hovered entity
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


// Get location, index information, and card value from card entity
//   Brute force (iterates through every table), could be sped up with some sort of hashing scheme but require a lot of transactional logic/time whenever a card is moved.
//   Maybe could add a Location component to each card so we don't have to check all the cards in every table
fn locate_card (
    map: &Res<TableMap>,
    tables: &Query<&AnimationTable>,
    entity: Entity
) -> Option<FocusedCardData> {
    // Loop over every table
    for (location, table_entity) in &map.0 {
        let table = tables.get(*table_entity).unwrap();
        
        // Check if card is in table
        if let Some(hand_index) = table.actual_index(entity) {

            // Extract other info
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
    None // Not a card entity
}

// Recalculates and updates mouse offsets when a new card is focused or a card is unfocused
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

    // Iterate over all tables that are sorted.  Only sorted tables have a mouse over effect. 
    //   TODO: Could use table map and hand_id to get just the local players hand
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
                // If the focused card is in an unsorted table
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

// Set the offset to zero and the scale to one for all entities iterated over
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

// Calculate appropriate highlighting offsets for each card
fn calculate_offsets <'a> (
    mouse_offsets: &mut Query<&mut MouseOffset>,
    entities: impl Iterator<Item = &'a Entity>,
    num_cards: usize,
    focused_index: usize, // The sorted index, not actual hand position
) {
    // Computes how far to shift cards to the side to make room for focused card size increase
    let offset = horizontal_offset(num_cards);

    for (i, entity) in entities.enumerate() {
        let mut mouse_offset = mouse_offsets.get_mut(*entity).unwrap();
        
        if i == focused_index {
            // Scale up focused card
            mouse_offset.offset = HIGHLIGHT_Y_OFFSET * Vec3::Y;
            mouse_offset.scale = HIGHLIGHT_SCALE;
        } else {
            // Shift non-focused cards to side
            let offset_dir = if i > focused_index {1.} else {-1.};
            mouse_offset.offset = offset * offset_dir * Vec3::X;
            mouse_offset.scale = 1.;
        }
    }
}