use dos_shared::cards::Card;
use dos_shared::table::*;

use super::MultiplayerState;

use bevy::prelude::*;

// TODO: move to interface manager?
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
                location: Location::Hand{player_id},
                card_value,
            });

            delay_total += delay_delta;
        }
    );

    // Flip card(s) to discard pile
    for card in to_discard_pile.iter() {
        commands.spawn().insert(DelayedDealtCard {
            timer: Timer::from_seconds(delay_total, false),
            location: Location::DiscardPile,
            card_value: Some(*card),
        });

        delay_total += delay_delta;
    }
}

// TODO: simplify this
#[derive(Component)]
pub struct DelayedDealtCard {
    pub timer: Timer,
    location: Location,
    pub card_value: Option<Card>,
}

use crate::game::table::CardTransferer;

pub fn delayed_dealing_system (
    mut query: Query<(Entity, &mut DelayedDealtCard)>,
    mut commands: Commands,
    mut card_transferer: CardTransferer,
    time: Res<Time>,
) {

    for (entity, mut delayed_card) in query.iter_mut() {
        delayed_card.timer.tick(time.delta());

        if delayed_card.timer.finished() {

            // TODO: simplify this
            card_transferer.transfer(
                CardReference{location: Location::Deck, index: None},
                CardReference{location: delayed_card.location, index: None},
                delayed_card.card_value,
            );
            
            commands.entity(entity).remove::<DelayedDealtCard>();
        }
    }
}
