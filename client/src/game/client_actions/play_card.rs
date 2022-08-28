use dos_shared::cards::Card;
use dos_shared::dos_game::DosGame;
use dos_shared::table::{CardReference, Location, HandPosition};
use dos_shared::messages::game::{FromClient, GameAction};

use crate::game::networking::GameNetworkManager;
use crate::game::table::FocusedCard;

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
) {
    if network_manager.card_tracker.has_delayed_transfers() {
        return;
    }

    for event in events.iter() {
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

fn handle_play_card(
    network_manager: &mut GameNetworkManager,
    card_reference: &CardReference,
) {
    let player = network_manager.card_tracker.mp_state.turn_id;

    let mut action = None;

    match network_manager.card_tracker.get_turn_state() {
        dos_shared::dos_game::TurnState::Default => {
            match card_reference.location {
                Location::Deck => {
                    if network_manager.card_tracker.validate_draw_cards(player) {
                        //network_manager.card_tracker.draw_cards();

                        action = Some(GameAction::DrawCards);
                    }
                },
                Location::Hand { player_id } if player_id == player =>  {
                    dbg!(player, card_reference);
                    dbg!(network_manager.card_tracker.game_info().current_turn());
                    
                    if network_manager.card_tracker.validate_play_card(player,card_reference) {
                        network_manager.card_tracker.play_card(card_reference);

                        action = Some(GameAction::PlayCard(*card_reference));
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

                    if network_manager.card_tracker.validate_play_card(player,&staging_reference) {
                        network_manager.card_tracker.play_card(&staging_reference);

                        action = Some(GameAction::PlayCard(staging_reference));
                    }
                },
                Location::Hand { player_id } if player_id == player => {
                    if network_manager.card_tracker.validate_keep_last_drawn_card(player) {
                        network_manager.card_tracker.keep_last_drawn_card();

                        action = Some(GameAction::KeepStaging);
                    }
                },
                Location::Staging => {
                    if network_manager.card_tracker.validate_play_card(player, card_reference) {
                        network_manager.card_tracker.play_card(card_reference);

                        action = Some(GameAction::PlayCard(*card_reference));
                    }
                },
                _ => {},
            }
        },
        _ => {},
    }

    // TODO: Could just embed this in match statement
    if let Some(action) = action {
        network_manager.send_message(FromClient(action));
    }
}