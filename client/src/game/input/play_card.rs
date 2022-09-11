use dos_shared::{
    dos_game::DosGame, 
    table::{CardReference, Location, HandPosition}, 
    messages::game::{FromClient, GameAction}
};

use crate::game::{
    graphics::FocusedCard, 
    networking::GameNetworkManager
};

use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;

// Runs on PickingEvent
pub fn play_card_system (
    mut network_manager: GameNetworkManager,
    mut events: EventReader<PickingEvent>,
    focused_card: Res<FocusedCard>,
) {
    // Do not allow player to submit an action until the animation state has caught up to the actual game state
    if network_manager.game.has_delayed_transfers() {
        return;
    }

    // Iterate over all picking events
    for event in events.iter() {

        // If there was a click event, then get the clicked on card data
        if let PickingEvent::Clicked(_) = event {
            if let Some(focused_card_data) = &focused_card.0 {

                let card_reference = CardReference {
                    location: focused_card_data.location,
                    hand_position: HandPosition::Index(focused_card_data.hand_index),
                };
                
                handle_play_card(&mut network_manager, &card_reference);

            } 
        }
    }
}

// Handles the player clicking on a card
// Different card location selections have different effects depending on the turn state.
// This function maps turn state and clicked on card to a game action.
// If the game action is valid, it updates the local state and sends the action to the server.
fn handle_play_card(
    network_manager: &mut GameNetworkManager,
    card_reference: &CardReference, // The selected card
) {
    let player = network_manager.game.mp_state.turn_id;

    match network_manager.game.get_turn_state() {
        dos_shared::dos_game::TurnState::TurnStart => {
            match card_reference.location {
                Location::Deck => {
                    if network_manager.game.validate_draw_cards(player) {
                        network_manager.send_message(FromClient(GameAction::DrawCards));
                    }
                },
                Location::Hand { player_id } if player_id == player =>  {
                    if network_manager.game.validate_play_card(player,card_reference) {
                        network_manager.game.play_card(card_reference);
                        network_manager.send_message(FromClient(GameAction::PlayCard(*card_reference)));
                    }
                },
                _ => {},
            }
        },
        dos_shared::dos_game::TurnState::StagedCard => {
            match card_reference.location {
                Location::DiscardPile => {
                    let staging_reference = CardReference {
                        location: Location::Staging,
                        hand_position: HandPosition::Last,
                    };

                    if network_manager.game.validate_play_card(player,&staging_reference) {
                        network_manager.game.play_card(&staging_reference);
                        network_manager.send_message(FromClient(GameAction::PlayCard(staging_reference)));
                    }
                },
                Location::Hand { player_id } if player_id == player => {
                    if network_manager.game.validate_keep_last_drawn_card(player) {
                        network_manager.game.keep_last_drawn_card();
                        network_manager.send_message(FromClient(GameAction::KeepStaging));
                    }
                },
                Location::Staging => {
                    if network_manager.game.validate_play_card(player, card_reference) {
                        network_manager.game.play_card(card_reference);
                        network_manager.send_message(FromClient(GameAction::PlayCard(*card_reference)));
                    }
                },
                _ => {},
            }
        },
        _ => {},
    }
}