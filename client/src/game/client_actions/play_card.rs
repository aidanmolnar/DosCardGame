use dos_shared::cards::Card;
use dos_shared::table::{CardReference, Location};
use dos_shared::valid_move;
use dos_shared::messages::game::FromClient;

use crate::game::networking::YourTurn;
use crate::game::table::FocusedCard;
use crate::game::table::CardTransferer;
use crate::multiplayer::MultiplayerState;
use crate::game::server_actions::dealing::DelayedDealtCard;

use std::io::Write;

use bevy::prelude::*;
use bevy_mod_picking::*;

#[derive(Component)]
pub struct CardValue (pub Card);

// Runs on PickingEvent

// TODO: Break this up and make it more readable
pub fn play_card_system (
    mp_state: Res<MultiplayerState>,
    mut events: EventReader<PickingEvent>,
    focused_card: Res<FocusedCard>,
    mut card_transferer: CardTransferer,
    your_turn: Option<Res<YourTurn>>,
    mut commands: Commands,
    delayed_cards: Query<&DelayedDealtCard>,
) {
    if your_turn.is_none() || !delayed_cards.is_empty() {
        return;
    }

    for event in events.iter() {
        if let PickingEvent::Clicked(_) = event {

            if let Some((location, table_index_data)) = &focused_card.0 {

                if let Some(card_value) = table_index_data.get_card_value() {
                    if let Some(discard_value) = card_transferer.peek_discard() {
                        println!("{:?}", discard_value);

                        if valid_move(card_value, discard_value) && 
                            *location == (Location::Hand{player_id: mp_state.turn_id})
                        {

                            card_transferer.transfer(
                                table_index_data.to_card_reference(*location), 
                                CardReference{location: Location::DiscardPile, index: None}, 
                                Some(card_value),
                            );
                            commands.remove_resource::<YourTurn>();

                            println!("{:?}", FromClient::PlayCard{card: table_index_data.to_card_reference(*location)});


                            // NOTE: Using bincode::serialize_into was causing crashes related to enum discriminants
                            let message = FromClient::PlayCard{card: table_index_data.to_card_reference(*location)};
                            mp_state.stream.as_ref().unwrap()
                            .write_all(
                                &bincode::serialize(&message).unwrap()
                            ).expect("Failed to send message");
                        }
                    }
                }
                
            }
        }
    }
}