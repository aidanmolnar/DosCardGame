use dos_shared::{cards::Card, messages::game};

use super::MultiplayerState;
use super::targeting::BoardPosition;
use super::components::LinearAnimation;
use super::graphics::layout::constants::*;

mod local_player_state;
mod animation_entity_tracker;
mod entity_manipulation;

use local_player_state::LocalPlayerState;
use animation_entity_tracker::{AnimationEntityTracker, HandOfCards};

use bevy::prelude::*;

// TODO: does this name make sense?
#[derive(Default)]
pub struct GameManager {
    pub player_id: usize, 
    local_player: LocalPlayerState,
    pub tracker: AnimationEntityTracker,

    // TODO: should this be somewhere else?  Why? any function with game manager needs this information...
    board_centers: Vec<(f32,f32)>,
}

// Can we set targets here?
impl GameManager {
    pub fn deal_to_you(
        &mut self,
        commands: &mut Commands,
        card: Card,
    ) {
        let hand_position = self.local_player.receive_card(card);
        
        // Spawn new card with MouseOverOffset, Pickable, SpriteSheet,
        // PrimaryTarget, LinearAnimator added later
        let entity = entity_manipulation::deal_to_you (
            commands,
            card
        );

        self.tracker.deal_card(self.player_id, Some(hand_position), entity);
        
        // Recompute relevent card target positions
        self.update_your_targets(commands);
    }

    pub fn deal_to_opponent(
        &mut self,
        commands: &mut Commands,
        player_id: usize,
    ) {
        // Spawn new card with SpriteSheet
        // PrimaryTarget, LinearAnimator added later
        let entity = entity_manipulation::deal_to_opponent(commands);

        self.tracker.deal_card(player_id, None, entity);

        // Recompute relevent card target positions
        self.update_opponent_targets(commands, player_id)
    }

    pub fn deal_to_discard_pile(
        &mut self,
        commands: &mut Commands,
        card: Card,
    ) {
        // Spawn new card with SpriteSheet
        // PrimaryTarget, LinearAnimator added later
        let entity = entity_manipulation::deal_to_discard(commands, card);

        self.tracker.discard_card(entity);

        // Recompute relevent card target positions
        self.update_discarded_card_target(commands, entity);
    }

    pub fn you_play_card(
        &mut self,
        commands: &mut Commands,
        card: Card,
    ) {
        let hand_position = self.local_player.play_card(card);
        let entity = self.tracker.play_card(self.player_id, hand_position);

        // Remove the MouseOverOffset, Pickable components
        entity_manipulation::you_play_card(commands, entity);

        // Recompute relevent card target positions
        self.update_your_targets(commands);
        self.update_discarded_card_target(commands, entity);
    }

    pub fn opponent_play_card(
        &mut self,
        commands: &mut Commands,
        player_id: usize,
        hand_position: usize,
        card: Card,
    ) {
        let entity = self.tracker.play_card(player_id, hand_position);

        // Change sprite sheet index
        entity_manipulation::opponent_play_card(commands, entity, card);

        // Recompute relevent card target positions
        self.update_opponent_targets(commands, player_id);
        self.update_discarded_card_target(commands, entity);
    }

    // TODO: finish this function
    pub fn reshuffle_deck(
        &mut self,
        commands: &mut Commands,
    ) {
        // Change the sprite sheet index to blanks
        // Animate them back to deck one at a time?

        // tracker.clear_discard_pile()
        // Would need to get discard pile from tracker?  Maybe this should return and then clear?

        //board_state.clear_discard_pile()
    }

    //TODO: rename.  Should these functions be somewhere else?

    fn update_your_targets (
        &self,
        commands: &mut Commands,
    ) {
        let hand = &self.tracker.hands.get(self.player_id).unwrap().0;

        let max_width =  your_max_hand_width(hand.len());
        let center = self.board_centers.get(self.player_id).unwrap();

        for (hand_position, entity) in hand.iter().enumerate() {

            let pos = max_width * arange_1d(hand.len(), hand_position); 
            let end = Vec3::new(center.0 + pos, center.1, 2. + (hand_position as f32) / 10.);

            commands.entity(*entity)
                .insert(BoardPosition {
                    position: end 
                });
        }

        println!("spawn");
        commands.spawn()
            .insert(AnimateCards::YourHand);
    }

    // TODO: reduce duplicate code: code for adding position / updating animation could be turned into function
    fn update_opponent_targets(
        &self,
        commands: &mut Commands,
        player_id: usize
    ) {
        let hand = &self.tracker.hands.get(player_id).unwrap().0;

        let max_width = opponent_max_hand_width(hand.len());
        let center = self.board_centers.get(player_id).unwrap();

        for (hand_position, entity) in hand.iter().enumerate() {

            let pos = max_width * arange_1d(hand.len(), hand_position); 
            let end = Vec3::new(center.0 + pos, center.1, 2. + (hand_position as f32) / 10.);

            commands.entity(*entity)
                .insert(BoardPosition {
                    position: end 
                });
        }

        println!("spawn");
        commands.spawn()
                .insert(AnimateCards::OpponentHand{player_id});
    }

    fn update_discarded_card_target (
        &self,
        commands: &mut Commands,
        entity: Entity,
    ) {
        let end = Vec3::new(DISCARD_LOCATION.0,DISCARD_LOCATION.1,0.1 * self.tracker.discard_pile.len() as f32);

        commands.entity(entity)
            .insert(BoardPosition {
                position: end 
            });
        
        println!("spawn");
        commands.spawn()
            .insert(AnimateCards::SingleCard(entity));
    }

    // TODO: Move focus updating code to here 
    // TODO: Add animation blueprint system
    fn update_your_focus(
        &self,
        commands: Commands,
    ) {
        let hand = &self.tracker.hands.get(self.player_id).unwrap().0;

    }
}

#[derive(Component)]
pub struct MouseOffset {
    offset: Vec3,
    scale: bool,
}

pub fn your_max_hand_width(hand_size: usize) -> f32 {
    f32::min(MAX_HAND_WIDTH, hand_size as f32 * MAX_HAND_SPACING)
}

fn opponent_max_hand_width(hand_size: usize) -> f32 {
    f32::min(MAX_OPPONENT_HAND_WIDTH, hand_size as f32 * MAX_HAND_SPACING)
}

// Returns a value between -0.5 and 0.5 based on position in array
pub fn arange_1d(len: usize, i: usize) -> f32 {
    if len > 1 {
        (i as f32 / (len-1) as f32) - 0.5
    } else {
         0.
    }
}

// Returns an (x,y) pair on unit circle between -angle / 2 and angle / 2 based on position in array
pub fn arange_arc(len: usize, i: usize, angle: f32) -> (f32, f32) {
    f32::sin_cos(angle * arange_1d(len, i))
}

// TODO: split this up, make it more readable
pub fn setup_game_manager (
    mp_state: Res<MultiplayerState>,
    mut game_manager: ResMut<GameManager>,
) {
    game_manager.player_id = mp_state.turn_id as usize;

    let num_players = mp_state.player_names.len() as u8;
    let num_other_players = num_players - 1;

    for _ in 0..num_players {
        game_manager.tracker.hands.push(HandOfCards(Vec::new()))
    }

    for owner_id in 0..num_players {

        if owner_id == mp_state.turn_id{
            game_manager.board_centers.push(YOUR_HAND_CENTER);
        } else {
            // Adjust other ids so your hand is skipped
            let local_id = if owner_id > mp_state.turn_id{
                (owner_id-1) % num_other_players
            } else {
                owner_id % num_other_players
            };
        
            // Arrange centers of opponents hands in an arc
            let (x,y) = arange_arc(
                num_other_players as usize, 
                local_id as usize,
                OPPONENT_ARC_ANGLE);
            let center_x = OPPONENT_ARC_WIDTH*x;
            let center_y = OPPONENT_ARC_HEIGHT*y;

            game_manager.board_centers.push( (center_x,center_y));
        }
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

#[derive(Component, Debug)]
pub enum AnimateCards {
    SingleCard (Entity),
    YourHand,
    OpponentHand {player_id: usize},
}

use super::mouse_over::FocusedCard;

use super::graphics::assets::*;

const HIGHLIGHT_SCALE: f32 = 1.25;

const WIDTH_OFFSET: f32 = (CARD_WIDTH * HIGHLIGHT_SCALE + CARD_WIDTH) / 2.;
const HIGHLIGHT_Y_OFFSET: f32 = CARD_HEIGHT * (HIGHLIGHT_SCALE - 1.) / 2.;

fn horizontal_offset(hand_size: usize) -> f32 {
    let hand_spacing = your_max_hand_width(hand_size) / (hand_size - 1) as f32; 
    WIDTH_OFFSET - hand_spacing
}

// Run on event only...
pub fn card_animation_system(
    animate_query: Query<(Entity, &AnimateCards)>,
    query: Query<(Option<&Transform>, &BoardPosition)>,
    game_manager: Res<GameManager>,
    focused_card: Res<FocusedCard>,
    mut commands: Commands,
) {
    for (animate_entity, animate_cards) in animate_query.iter() {
        println!("run");

        match animate_cards {
            AnimateCards::SingleCard(entity) => {
                animate_single_card(&mut commands, &query, entity);
            }
            AnimateCards::YourHand => {
                if let Some(focused_entity) = focused_card.0 {
                    animate_focused_hand(&mut commands, &query, &game_manager, focused_entity);
                } else {
                    animate_unfocused_hand(&mut commands, &query, &game_manager, game_manager.player_id);
                }
            }
            AnimateCards::OpponentHand{player_id} => {
                animate_unfocused_hand(&mut commands, &query, &game_manager, *player_id);
            }
        }

        commands.entity(animate_entity).despawn();
    }
}

fn animate_single_card (
    commands: &mut Commands,
    query: &Query<(Option<&Transform>, &BoardPosition)>,
    entity: &Entity,
) {
    let (transform, position) = query.get(*entity).unwrap();

    commands.entity(*entity)
    .insert(
        LinearAnimation {
            start: *transform.unwrap_or(&Transform::from_translation(Vec3::new(DECK_LOCATION.0,DECK_LOCATION.1,0.))),
            end: Transform::from_translation(position.position),
            timer: Timer::from_seconds(0.1, false),
        }
    );
}

fn animate_focused_card (
    commands: &mut Commands,
    query: &Query<(Option<&Transform>, &BoardPosition)>,
    entity: &Entity,
) {
    let (transform, position) = query.get(*entity).unwrap();

    commands.entity(*entity)
    .insert(
        LinearAnimation {
            start: *transform.unwrap_or(&Transform::from_translation(Vec3::new(DECK_LOCATION.0,DECK_LOCATION.1,0.))),
            end: Transform {
                translation: position.position + HIGHLIGHT_Y_OFFSET * Vec3::Y,
                scale: Vec3::splat(HIGHLIGHT_SCALE),
                ..default()
            },
            timer: Timer::from_seconds(0.1, false),
        }
    );
}

fn animate_offset_card (
    commands: &mut Commands,
    query: &Query<(Option<&Transform>, &BoardPosition)>,
    entity: &Entity,
    offset: f32,
) {
    let (transform, position) = query.get(*entity).unwrap();
    // let (transform, position) = match query.get(*entity) {
    //     Ok(v) => v,
    //     Err(_) => {println!("E"); return},
    // };

    commands.entity(*entity)
    .insert(
        LinearAnimation {
            start: *transform.unwrap_or(&Transform::from_translation(Vec3::new(DECK_LOCATION.0,DECK_LOCATION.1,0.))),
            end: Transform {
                translation: position.position + offset*Vec3::X,
                scale: Vec3::splat(1.),
                ..default()
            },
            timer: Timer::from_seconds(0.1, false),
        }
    );
}

fn animate_unfocused_hand(
    commands: &mut Commands,
    query: &Query<(Option<&Transform>, &BoardPosition)>,
    game_manager: &Res<GameManager>,
    player_id: usize,
) {
    let hand = &game_manager.tracker.hands.get(player_id).unwrap().0;

    for entity in hand.iter() {
        animate_single_card(commands, query, entity);
    }
}

fn animate_focused_hand (
    commands: &mut Commands,
    query: &Query<(Option<&Transform>, &BoardPosition)>,
    game_manager: &Res<GameManager>,
    focused_entity: Entity,
) {
    let hand = &game_manager.tracker.hands.get(game_manager.player_id).unwrap().0;

    let focused_index = hand.iter().position(|&e| e == focused_entity).unwrap();
    let offset = horizontal_offset(hand.len());

    for (i, entity) in hand.iter().enumerate() {
        if i == focused_index {
            animate_focused_card(commands, query, entity);
        } else {
            let offset_sign = (i as isize - focused_index as isize).signum();
            animate_offset_card(commands, query, entity, offset * offset_sign as f32)
        }
    }
}