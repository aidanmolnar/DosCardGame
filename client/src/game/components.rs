
use bevy::prelude::*;



#[derive(Component)]
pub struct LinearAnimation {
    pub start: Transform,
    pub end: Transform,
    pub timer: Timer,
}
