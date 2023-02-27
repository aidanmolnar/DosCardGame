use super::components::{BoardPosition, MouseOffset};

use bevy::prelude::*;

pub fn remove_all_cards(
    mut commands: Commands,
    cards: Query<Entity, (With<BoardPosition>, With<MouseOffset>)>,
) {
    for entity in &cards {
        commands.entity(entity).despawn_recursive();
    }
}
