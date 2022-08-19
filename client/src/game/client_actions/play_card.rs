use dos_shared::cards::Card;
use dos_shared::table::{CardReference, Location};
use dos_shared::valid_move;
use dos_shared::messages::game::FromClient;

use crate::game::networking::GameNetworkManager;
use crate::game::table::{FocusedCard, TableIndexData};
use crate::game::server_actions::dealing::DelayedTransfer;

use bevy::prelude::*;
use bevy_mod_picking::*;

#[derive(Component)]
pub struct CardValue (pub Card);

// Runs on PickingEvent

// TODO: Break this up and make it more readable
pub fn play_card_system (
    mut network_manager: GameNetworkManager,
    mut events: EventReader<PickingEvent>,
    focused_card: Res<FocusedCard>,
    delayed_cards: Query<&DelayedTransfer>,
) {
    if !network_manager.is_your_turn() || !delayed_cards.is_empty() {
        return;
    }


    // TODO: Clean this up.  Remove TableIndexData
    for event in events.iter() {
        if let PickingEvent::Clicked(_) = event {

            if let Some((location, table_index_data)) = &focused_card.0 {

                // Could have a match based on location here
                match location {
                    Location::Hand{player_id} if *player_id == network_manager.mp_state.turn_id => {
                        if let Some(card) = network_manager.card_transferer.peek_staging() {

                            network_manager.send_message(FromClient::KeepStaging);

                            let from = CardReference{
                                location: Location::Staging,
                                index: None
                            };
                            let to = CardReference{
                                location: Location::Hand{player_id: network_manager.mp_state.turn_id},
                                index: None
                            };
                            
                            network_manager.card_transferer.transfer(from, to, Some(card));
                            network_manager.game_info.next_turn();
                        } else {
                            play_card(
                                &mut network_manager, 
                                location, 
                                table_index_data
                            );
    
                        }
                    }
                    Location::Deck => {
                        if network_manager.card_transferer.peek_staging().is_none() {
                            network_manager.send_message(FromClient::DrawCards);
                        }
                
                        //network_manager.game_info.next_turn();
                    }
                    Location::DiscardPile | Location::Staging => {
                        if let Some(card) = network_manager.card_transferer.peek_staging() {
                            // TODO: this is really hacky, we know the card is playable.  Play card should be split up
                            play_card(
                                &mut network_manager, 
                                &Location::Staging, 
                                &TableIndexData::Sorted{ hand_position: 0, sorted_position: 0, card_value: card}
                            )
                        }
                    }
                    _ => {}
                }
            } 
        }
    }
}


fn play_card(
    network_manager: &mut GameNetworkManager,
    location: &Location,
    table_index_data: &TableIndexData,
) {

    // Get the selected card and the top of the discard pile
    if let Some(card_value) = table_index_data.get_card_value() {
        if let Some(discard_value) = network_manager.card_transferer.peek_discard() {
        
            // Make sure the move is legal
            if valid_move(card_value, discard_value) {

                // Update the local client instantly
                network_manager.card_transferer.transfer(
                    table_index_data.to_card_reference(location), 
                    CardReference{location: Location::DiscardPile, index: None}, 
                    Some(card_value),
                );
                network_manager.game_info.next_turn();

                // Tell the server the intended move
                network_manager.send_message(FromClient::PlayCard{card: table_index_data.to_card_reference(location)});
                
            }
        }
    }
}