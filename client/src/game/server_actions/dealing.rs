use dos_shared::cards::Card;
use dos_shared::table::*;

use crate::game::table::CardTransferer;
use super::MultiplayerState;

use bevy::prelude::*;

// TODO: move somewhere
// TODO: A bit memory inefficient creating this vec to store all the values, there's probably a way to do this functionally or with an iterator
pub fn deal_out_cards(
    your_cards: Vec<Card>, 
    deck_size: usize,
    to_discard_pile: Vec<Card>,
    commands: &mut Commands,
    mp_state: &mut ResMut<MultiplayerState>,
) {
    
    let mut card_index = 0;

    let mut transfers = Vec::new();

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

            transfers.push(CardTransfer {
                from: CardReference{location: Location::Deck, index: None},
                to: CardReference{location: Location::Hand{player_id}, index: None},
                value: card_value,
            });
        }
    );

    // Flip card(s) to discard pile
    for card in to_discard_pile.iter() {
        transfers.push(CardTransfer {
            from: CardReference{location: Location::Deck, index: None},
            to: CardReference{location: Location::DiscardPile, index: None},
            value: Some(*card),
        });
    }

    create_delayed_transfers(commands, transfers, 0.1)
}

// TODO: simplify this
#[derive(Component)]
pub struct DelayedTransfer {
    pub timer: Timer,
    pub transfer: CardTransfer,
}


pub fn delayed_dealing_system (
    mut query: Query<(Entity, &mut DelayedTransfer)>,
    mut commands: Commands,
    mut card_transferer: CardTransferer,
    time: Res<Time>,
) {

    for (entity, mut delayed_transfer) in query.iter_mut() {
        delayed_transfer.timer.tick(time.delta());

        if delayed_transfer.timer.finished() {

            // TODO: simplify this
            card_transferer.transfer(
                delayed_transfer.transfer.from,
                delayed_transfer.transfer.to,
                delayed_transfer.transfer.value,
            );
            
            commands.entity(entity).remove::<DelayedTransfer>();
        }
    }
}

pub fn create_delayed_transfers(
    commands: &mut Commands,
    transfers: Vec<CardTransfer>,
    delay_delta: f32,
) {
    let mut delay_total: f32 = 0.0;

    for transfer in transfers {
        commands.spawn().insert(DelayedTransfer {
            timer: Timer::from_seconds(delay_total, false),
            transfer,
        });

        delay_total += delay_delta;
    }
}