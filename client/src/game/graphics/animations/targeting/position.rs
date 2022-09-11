use dos_shared::table::Table;

use super::{
    core::components::BoardPosition, 
    AnimationTable,
    layout::{
        expressions::arange_1d,
        constants::MAX_HAND_SPACING
    }};

use bevy::prelude::*;

// Stores information about how to position cards in the table
#[derive(Component)]
pub struct TableArranger {
    pub center: (f32,f32),
    pub max_width: f32,
}

// Update target final position of cards in tables that have been changed by a transfer
pub fn update_system (
    tables: Query<(&AnimationTable, &TableArranger), Changed<AnimationTable>>,
    mut cards: Query<&mut BoardPosition>,
    
) {
    for (table, arranger) in tables.iter() {
        calculate_positions(
            &mut cards,
            arranger,
            table.iter_entities(), 
            table.len() 
        );
    }
}

// Horizontally spreads out cards in a table
fn calculate_positions<'a> (
    cards: &mut Query<&mut BoardPosition>,
    arranger: &TableArranger,
    entities: impl Iterator<Item = &'a Entity>,
    num_cards: usize,
) {
    // Tries to spread out cards by MAX_HAND_SPACING unless that would push the cards past arranger.max_width
    #[allow(clippy::cast_precision_loss)]
    let width = f32::min(arranger.max_width, num_cards as f32 * MAX_HAND_SPACING);

    for (hand_position, entity) in entities.enumerate() {
        let mut board_position = cards.get_mut(*entity).unwrap();

        let pos = width * arange_1d(num_cards, hand_position); 
        
        #[allow(clippy::cast_precision_loss)]
        let z = 2. + (hand_position as f32) / 100.;
        board_position.position = Vec3::new(arranger.center.0 + pos, arranger.center.1, z);
    }
}