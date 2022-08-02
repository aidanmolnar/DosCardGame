use bevy::prelude::*;

#[derive(Component)]
pub struct MouseOffset {
    pub offset: Vec3,
    pub scale: f32,
}

#[derive(Component)]
pub struct BoardPosition {
    pub position: Vec3
}

#[derive(Component)]
pub struct LinearAnimation {
    pub start: Transform,
    pub end: Transform,
    pub timer: Timer,
}