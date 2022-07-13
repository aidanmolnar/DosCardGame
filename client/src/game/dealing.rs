use dos_shared::cards::Card;

use super::MultiplayerState;
use super::GameManager;

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

// TODO: simplify this
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
    mut game_manager: ResMut<GameManager>,
    time: Res<Time>,
) {
    for (entity, mut delayed_card) in query.iter_mut() {
        delayed_card.timer.tick(time.delta());

        if delayed_card.timer.finished() {

            // TODO: simplify this
            if delayed_card.owner_id as usize == game_manager.tracker.player_id {
                game_manager.deal_to_you(&mut commands, delayed_card.card_value.unwrap());
            } else if delayed_card.owner_id == 255 {
                game_manager.deal_to_discard_pile(&mut commands, delayed_card.card_value.unwrap());
            } else {
                game_manager.deal_to_opponent(&mut commands, delayed_card.owner_id as usize);
            }
            
            commands.entity(entity).remove::<DelayedDealtCard>();
        }
    }
}
