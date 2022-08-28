use dos_shared::transfer::Table;

use crate::game::animations::components::*;
use crate::game::layout::{expressions::*,constants::*};
use super::ClientTable;


use bevy::prelude::*;

#[derive(Component)]
pub struct TableArranger {
    pub center: (f32,f32),
    pub max_width: f32,
}

pub fn update_system (
    tables: Query<(&ClientTable, &TableArranger), Changed<ClientTable>>,
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

fn calculate_positions<'a> (
    cards: &mut Query<&mut BoardPosition>,
    arranger: &TableArranger,
    entities: impl Iterator<Item = &'a Entity>,
    num_cards: usize,
) {
    let width = f32::min(arranger.max_width, num_cards as f32 * MAX_HAND_SPACING);

    for (hand_position, entity) in entities.enumerate() {
        let mut board_position = cards.get_mut(*entity).unwrap();

        let pos = width * arange_1d(num_cards, hand_position); 
        
        board_position.position = Vec3::new(arranger.center.0 + pos, arranger.center.1, 2. + (hand_position as f32) / 100.);
    }
}