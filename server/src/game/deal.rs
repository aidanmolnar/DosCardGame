use dos_shared::{dos_game::DosGame, DECK_SIZE, messages::game::{GameAction, FromServer}};

use crate::multiplayer::{NetPlayer, Agent};

use super::networking::GameNetworkManager;

use bevy::prelude::*;

pub fn deal_out_starting_hands(
    mut network_manager: GameNetworkManager,
    query: Query<(Entity, &NetPlayer, &Agent)>,
) {
    network_manager.card_tracker.deal_starting_cards(DECK_SIZE);

    let conditions = network_manager.card_tracker.syncer.take_conditions();

    for (_, _, agent) in query.iter() {
        let cards = network_manager.card_tracker.syncer.take_player_cards(agent.turn_id);
        
        network_manager.send_to_one(
            &query, 
            FromServer {
                action: GameAction::DealIn, 
                conditions: conditions.clone(), 
                cards
            }, 
            agent.turn_id
        )
    }
}