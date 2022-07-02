



use super::setup_graphics::HandLocations;
use super::CardTracker;
use super::MultiplayerState;
use super::interface_constants::*;
use super::arange::arange_1d;

use bevy::prelude::*;
use bevy::ecs::event::Events;

// System for animating cards to their target locations
pub fn run_animations (
    mut query: Query<(&mut LinearAnimation, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut target, mut transform) in query.iter_mut() {

        target.timer.tick(time.delta());
        // LERP towards target end location
        transform.translation = target.start + (target.end - target.start) * target.timer.percent();
        
    }
}  

#[derive(Component)]
pub struct LinearAnimation {
    pub start: Vec3,
    pub end: Vec3,
    pub timer: Timer,
}

pub struct HandUpdated {
    pub owner_id: u8
}

pub fn set_card_targets (
    mut query: Query<(&mut LinearAnimation, &Transform)>,
    other_locations: Res<HandLocations>,
    card_tracker: Res<CardTracker>,
    mut events: ResMut<Events<HandUpdated>>,
    mp_state: Res<MultiplayerState>,
) {
    for event in events.drain() {

        let hand = card_tracker.map.get(event.owner_id as usize).unwrap();
        let (center_x, center_y) = other_locations.centers.get(event.owner_id as usize).unwrap();

        let max_width = if event.owner_id == mp_state.turn_id {
            f32::min(MAX_HAND_WIDTH, hand.len() as f32 * MAX_HAND_SPACING)
        } else {
            f32::min(MAX_OPPONENT_HAND_WIDTH, hand.len() as f32 * MAX_HAND_SPACING)
        };

        for (hand_position, entity) in hand.iter().enumerate() {
            let (mut animation, transform) = query.get_mut(*entity).unwrap();
            let pos = max_width * arange_1d(hand.len(), hand_position); 
            let new_dest = Vec3::new(*center_x + pos, *center_y, 2. + (hand_position as f32) / 10.);

            animation.start = transform.translation;
            animation.end = new_dest;
            animation.timer = Timer::from_seconds(0.1, false);
        }
    }
}



