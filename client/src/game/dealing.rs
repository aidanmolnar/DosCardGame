use dos_shared::cards::Card;

use super::MultiplayerState;
use super::card_tracker::CardTracker;

// TODO: move functions that need these into graphics

use super::graphics::animations::HandUpdated;
use super::graphics::spawn_card::spawn_card_entity;

use bevy::prelude::*;

pub fn deal_out_cards(
    your_cards: Vec<Card>, 
    deck_size: usize,
    to_discard_pile: Vec<Card>,
    mut commands: Commands,
    mp_state: ResMut<MultiplayerState>,
) {
    let delay_delta = 0.25;
    let mut delay_total = 0.0;
    let mut card_index = 0;

    // Deals cards to players one at a time
    dos_shared::deal_cards(
        mp_state.player_names.len(),
        deck_size,
        |player_id| {
            let card_value = if player_id == mp_state.turn_id as usize {
                card_index += 1;
                Some(*your_cards.get(card_index-1).unwrap())
            } else {
                None
            };

            commands.spawn().insert(DelayedDealtCard {
                timer: Timer::from_seconds(delay_total, false),
                owner_id: player_id as u8,
                card_value,
                discarded: false,
            });

            delay_total += delay_delta;
        }
    );

    // Flip card(s) to discard pile
    for card in to_discard_pile.iter() {
        commands.spawn().insert(DelayedDealtCard {
            timer: Timer::from_seconds(delay_total, false),
            owner_id: 255,
            card_value: Some(*card),
            discarded: true,
        });

        delay_total += delay_delta;
    }
}

#[derive(Component)]
pub struct DelayedDealtCard {
    pub timer: Timer,
    pub owner_id: u8,
    pub card_value: Option<Card>,
    pub discarded: bool,
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
                delayed_card.into_inner(),
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
    delayed_dealt_card: &DelayedDealtCard,
    commands: &mut Commands,
    card_tracker: &mut ResMut<CardTracker>,
    events: &mut EventWriter<HandUpdated>,
    mp_state: &Res<MultiplayerState>,
) {
    let entity = spawn_card_entity(
        delayed_dealt_card.card_value,
        delayed_dealt_card.owner_id == mp_state.turn_id,
        delayed_dealt_card.discarded,
        commands,
    );
    
    if !delayed_dealt_card.discarded {
        // Add the card to the card tracker
        card_tracker.add_card(
            delayed_dealt_card.card_value,
            entity,
            delayed_dealt_card.owner_id,
            mp_state.turn_id,
        );

        // Sends event to update card target locations
        events.send(HandUpdated{owner_id: delayed_dealt_card.owner_id})
    }
    
}
