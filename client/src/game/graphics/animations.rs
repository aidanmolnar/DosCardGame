use super::setup_graphics::HandLocations;
use super::CardTracker;
use super::MultiplayerState;
use super::interface_constants::*;
use super::arange::arange_1d;
use super::GameState;
use super::assets::{CARD_WIDTH, CARD_HEIGHT};

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy::ecs::event::Events;
use bevy_mod_picking::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(run_animations
            .run_in_state(GameState::InGame))
        .add_system_to_stage(CoreStage::PreUpdate,
            set_card_targets
            .run_in_state(GameState::InGame)
            .run_on_event::<HandUpdated>())
        .add_system(your_hand_animation_system
            .run_on_event::<PickingEvent>());
    }
}

// System for animating cards to their target locations
pub fn run_animations (
    mut query: Query<(&mut LinearAnimation, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut target, mut transform) in query.iter_mut() {
        target.timer.tick(time.delta());
        // LERP towards target end location
        transform.translation = target.start.translation + (target.end.translation - target.start.translation) * target.timer.percent();
        transform.scale = target.start.scale + (target.end.scale - target.start.scale) * target.timer.percent();
    }
}  

#[derive(Component)]
pub struct LinearAnimation {
    pub start: Transform,
    pub end: Transform,
    pub timer: Timer,
}

impl LinearAnimation{
    pub fn set(&mut self, start: Transform, end: Transform, in_time: f32) {
        self.start = start;
        self.end = end;
        self.timer = Timer::from_seconds(in_time, false);
    }
    // pub fn set_translation (&mut self, start: Transform, end: Vec3, in_time: f32) {
    //     self.start = start;
    //     self.end.translation = end;
    //     self.timer = Timer::from_seconds(in_time, false);
    // }
}

pub struct HandUpdated {
    pub owner_id: u8
}

#[derive(Component)]
pub struct Target {
    pub target: Vec3,
}

// TODO: add comments, don't expect entity to have animation already, just defer processing the event to the next frame
pub fn set_card_targets (
    mut query: Query<(&mut Target,&mut LinearAnimation, &Transform)>,
    mut events: ResMut<Events<HandUpdated>>,
    other_locations: Res<HandLocations>,
    card_tracker: Res<CardTracker>,
    mp_state: Res<MultiplayerState>,
) {
    for event in events.drain() {

        let hand = card_tracker.map.get(event.owner_id as usize).unwrap();
        let (center_x, center_y) = other_locations.centers.get(event.owner_id as usize).unwrap();

        let max_width = if event.owner_id == mp_state.turn_id {
            your_max_hand_width(hand.len())
        } else {
            opponent_max_hand_width(hand.len())
        };

        for (hand_position, entity) in hand.iter().enumerate() {
            let (mut target, mut animation, transform) = query.get_mut(*entity).unwrap();

            let pos = max_width * arange_1d(hand.len(), hand_position); 

            target.target = Vec3::new(*center_x + pos, *center_y, 2. + (hand_position as f32) / 10.);
            animation.set(
                *transform,
                Transform {
                    translation: target.target,
                    scale: Vec3::splat(1.),
                    ..default()
                },
                0.1,
            );
        }
    }
}

fn your_max_hand_width(hand_size: usize) -> f32 {
    f32::min(MAX_HAND_WIDTH, hand_size as f32 * MAX_HAND_SPACING)
}
fn opponent_max_hand_width(hand_size: usize) -> f32 {
    f32::min(MAX_OPPONENT_HAND_WIDTH, hand_size as f32 * MAX_HAND_SPACING)
}

const HIGHLIGHT_SCALE: f32 = 1.25;

const WIDTH_OFFSET: f32 = (CARD_WIDTH * HIGHLIGHT_SCALE + CARD_WIDTH) / 2.;
const HIGHLIGHT_Y_OFFSET: f32 = CARD_HEIGHT * (HIGHLIGHT_SCALE - 1.) / 2.;

fn horizontal_offset(hand_size: usize) -> Vec3 {
    let hand_spacing = your_max_hand_width(hand_size) / (hand_size - 1) as f32; 
    (WIDTH_OFFSET - hand_spacing) * Vec3::X
}

// TODO: clean up, break up into smaller pieces, comment, etc
// TODO: Run this retargeting system after the set_card_targets somehow to fix jenky dealing behavior
//       Will need to find highlighted card from query instead of events
// Sets position / scale of cards in your hand based on mouse hovering
pub fn your_hand_animation_system (
    mut query: Query<(&Target, &mut LinearAnimation, &Transform)>,
    mut events: EventReader<PickingEvent>,
    card_tracker: Res<CardTracker>,
    mp_state: Res<MultiplayerState>,
) {
    let your_card_entities = card_tracker.map.get(mp_state.turn_id as usize).unwrap();

    // Get index of focused card
    let mut do_something = false;
    let mut option_index = None;

    for event in events.iter() {
        if let PickingEvent::Hover(hover_event) = event {
            match hover_event {
                HoverEvent::JustEntered(entity) => {
                    do_something = true;
                    option_index = your_card_entities.iter().position(|&e| e == *entity);
                }
                HoverEvent::JustLeft(_) => {
                    do_something = true;
                }
            }
        }
    }

    if !do_something {
        return;
    }

    if let Some(index) = option_index {

        let offset = horizontal_offset(your_card_entities.len());

        // Iterate over card tracker in ascending order
        for (i, entity) in your_card_entities.iter().enumerate() {
            if let Ok((target, mut animation, transform)) = query.get_mut(*entity) {
                if i == index {
                    animation.set(
                        *transform,
                        Transform {
                            translation: target.target + HIGHLIGHT_Y_OFFSET * Vec3::Y,
                            scale: Vec3::splat(HIGHLIGHT_SCALE),
                            ..default()
                        },
                        0.1,
                    );
                } else {
                    // compute offset from index of focused card
                    let offset_sign = (i as isize - index as isize).signum();
    
                    // apply a translation scaled by the offset
                    // TODO: Adjust this -> can lead to overlap of cards close to selected card making it impossible to select for large hand sizes
                    animation.set(
                        *transform,
                        Transform {
                            translation: target.target + offset * offset_sign as f32,
                            scale: Vec3::splat(1.),
                            ..default()
                        },
                        0.1,
                    );
                }
            }
        }
    } else {
        // Reset positions
        for entity in your_card_entities.iter() {
            if let Ok((target, mut animation, transform)) = query.get_mut(*entity) {
                animation.set(
                    *transform,
                    Transform {
                        translation: target.target,
                        ..default()
                    },
                    0.1,
                );
            }
        }
    }
}
