use dos_shared::cards::Card;

// TODO: these are awful imports
use super::components::*; 

// TODO: Should this exist?

use bevy::prelude::*;

// Functions for constructing or modifying card entities

// Deal to you
// Spawn new card with PrimaryTarget, MouseOverOffset, Pickable, SpriteSheet, LinearAnimator
pub fn deal_to_you(
    commands: &mut Commands,
    card: Card,
) -> Entity {
    commands
    .spawn()
    .insert(
        CardBlueprint {
        card: Some(card),
    })
    .insert(
        PickableBlueprint
    ).id()
}

// Deal to opponent
// Spawn new card with PrimaryTarget, SpriteSheet, LinearAnimator
pub fn deal_to_opponent(
    commands: &mut Commands
) -> Entity {
    commands
    .spawn()
    .insert(
        CardBlueprint {
        card: None,
    }).id()
}

// Deal to discard
// TODO: this is the same as dealing to opponent, can consolidate
// Spawn new card with PrimaryTarget, SpriteSheet, LinearAnimator
pub fn deal_to_discard(
    commands: &mut Commands,
    card: Card,
) -> Entity {
    commands
    .spawn()
    .insert(
        CardBlueprint {
        card: Some(card),
    }).id()
}

// You play card
// Remove the MouseOverOffset, Pickable components
// Add discard component or change position enum to discarded
pub fn you_play_card(
    commands: &mut Commands,
    entity: Entity,
) {
    //commands.entity(entity)
    // TODO: remove components that make the card pickable
}

// Opponent play card
// Add discard component or change position enum to discarded
pub fn opponent_play_card(
    commands: &mut Commands,
    entity: Entity,
    card: Card
) {
    commands.entity(entity)
    .insert(CardBlueprint {
        card: Some(card),
    });
}