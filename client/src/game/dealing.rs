use dos_shared::{cards::*, NUM_STARTING_CARDS};

use super::MultiplayerState;
use super::card_tracker::CardTracker;

// TODO: move functions that need these into graphics

use super::graphics::animations::HandUpdated;
use super::graphics::spawn_card::spawn_card_entity;

use bevy::prelude::*;

pub fn deal_out_cards(
    your_cards: Vec<Card>, 
    mut card_counts: Vec<u8>,
    mut commands: Commands,
    mp_state: ResMut<MultiplayerState>,
) {

    let delay_delta = 0.05;
    let mut delay_total = 0.0;

    // Deal out the hands from the deck
    // This is probably more complicated than it needs to be, can make assumptions about how server deals out cards.  Remove card counts from message?
    // TODO: Simplify
    for j in 0..NUM_STARTING_CARDS {
        for (card_owner_id,count) in card_counts.iter_mut().enumerate() {
            if *count > 0 {
                *count -= 1;

                let card_value = if card_owner_id == mp_state.turn_id as usize {
                    Some(*your_cards.get(j as usize).unwrap())
                } else {
                    None
                };

                commands.spawn().insert(DelayedDealtCard {
                    timer: Timer::from_seconds(delay_total, false),
                    owner_id: card_owner_id as u8,
                    card_value,
                });

                delay_total += delay_delta;
                
            } else {
                break;
            }
        }
    }
}

#[derive(Component)]
pub struct DelayedDealtCard {
    pub timer: Timer,
    pub owner_id: u8,
    pub card_value: Option<Card>,
}

pub fn delayed_dealing_system (
    mut query: Query<(Entity, &mut DelayedDealtCard)>,
    mut commands: Commands,
    mut card_tracker: ResMut<CardTracker>,
    mut events: EventWriter<HandUpdated>,
    mp_state: Res<MultiplayerState>,
    time: Res<Time>,
) {
    for (entity, mut delayed_card) in query.iter_mut() {
        delayed_card.timer.tick(time.delta());

        if delayed_card.timer.finished() {
            deal_card(
                delayed_card.owner_id,
                delayed_card.card_value,
                &mut commands,
                &mut card_tracker,
                &mut events,
                &mp_state,
            );
            commands.entity(entity).remove::<DelayedDealtCard>();
        }
    }
}

pub fn deal_card (
    owner_id: u8,
    card_value: Option<Card>,
    commands: &mut Commands,
    card_tracker: &mut ResMut<CardTracker>,
    events: &mut EventWriter<HandUpdated>,
    mp_state: &Res<MultiplayerState>,
) {
    
    let entity = spawn_card_entity(
        card_value,
        owner_id == mp_state.turn_id,
        commands,
    );
    
    // Add the card to the card tracker
    card_tracker.add_card(
        card_value,
        entity,
        owner_id,
        mp_state.turn_id,
    );

    // Sends event to update card target locations
    events.send(HandUpdated{owner_id})
}
