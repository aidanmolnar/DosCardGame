

use super::animations::components::*;
use super::make_tables::*;
use super::table::*;

use bevy::prelude::*;

pub fn update_table_system(
    tables: Query<(&ClientTable, &TableArranger), Changed<ClientTable>>,
    mut cards: Query<(&mut LinearAnimation, &Transform, &mut BoardPosition, Option<&MouseOffset>)>,
) {

    for (table, arranger) in tables.iter() {
        arrange_cards(
            &mut cards,
            arranger,
            table.iter(), 
            table.len() 
        );
    }
}
    
use super::layout::expressions::*;
use super::layout::constants::*;

use super::animations::systems::update_animation;

fn arrange_cards<'a> (
    cards: &mut Query<(&mut LinearAnimation, &Transform, &mut BoardPosition, Option<&MouseOffset>)>,
    arranger: &TableArranger,
    entities: impl Iterator<Item = &'a Entity>,
    num_cards: usize,
) {
    let width = f32::min(arranger.max_width, num_cards as f32 * MAX_HAND_SPACING);

    for (hand_position, entity) in entities.enumerate() {
        let (mut animation, 
            transform, 
            mut board_position, 
            mouse_option
        ) = cards.get_mut(*entity).unwrap();

        let pos = width * arange_1d(num_cards, hand_position); 
        board_position.position = Vec3::new(arranger.center.0 + pos, arranger.center.1, 2. + (hand_position as f32) / 10.);

        update_animation(
            &mut animation, 
            transform, 
            &board_position, 
            mouse_option
        );
    }
}