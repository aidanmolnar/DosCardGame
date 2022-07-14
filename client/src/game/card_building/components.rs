use dos_shared::cards::Card;

use bevy::prelude::Component;

#[derive(Component)]
pub struct CardBlueprint {
    pub card: Option<Card>,
}

#[derive(Component)]
pub struct PickableBlueprint;