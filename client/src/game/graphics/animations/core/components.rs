use bevy::prelude::*;

// Used for linearly interpolating bevy transforms from start to end
#[derive(Component)]
pub struct LinearAnimation {
    pub start: Transform,
    pub end: Transform,
    pub timer: Timer,
}

// The target location for an animated card
#[derive(Component)]
pub struct BoardPosition {
    pub position: Vec3
}

// Offset from target location due to mouse overs
#[derive(Component)]
pub struct MouseOffset {
    pub offset: Vec3,
    pub scale: f32,
}



