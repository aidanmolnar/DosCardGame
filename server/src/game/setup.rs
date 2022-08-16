use dos_shared::table::*;
use dos_shared::cards::*;
use dos_shared::messages::game::FromServer;

use super::game_info::GameInfo;
use super::multiplayer::{NetPlayer, Agent, AgentTracker};
use super::table::*;

use bevy::prelude::*;

pub const DECK_SIZE: usize = 108;

// TODO: break up into smaller pieces
pub fn deal_cards(
    query: Query<(&NetPlayer, &Agent)>,
    mut card_transferer: CardTransferer,
    agent_tracker: Res<AgentTracker>,
) {
    dos_shared::deal_cards(
        agent_tracker.agents.len(),
        DECK_SIZE,
        |player_id: usize| {
            card_transferer.transfer(
                CardReference {
                    location: Location::Deck, 
                    index: None 
                },
                CardReference {location: 
                    Location::Hand{player_id}, 
                    index: None 
                }
            );
        },
    );

    // Discards cards until a non wild one is found
    loop {
        let card = card_transferer.transfer(
            CardReference {
                location: Location::Deck, 
                index: None 
            },
            CardReference {location: 
                Location::DiscardPile, 
                index: None 
            }
        );

        match card.ty {
            CardType::Wild => {continue},
            CardType::DrawFour => {continue}
            _=> {break}
        }
    }


    // TODO: there may be a better/more functional way to do this that doesn't require cloning the hands
    let discard_pile = card_transferer.find_table(&Location::DiscardPile).0.clone();
    for (player, agent) in query.iter() {
        let table = card_transferer.find_table(&Location::Hand{player_id: agent.turn_id});

        if let Err(e) = bincode::serialize_into(
            &player.stream, 
            &FromServer::DealIn{
                your_cards: table.0.clone(),
                deck_size: DECK_SIZE,
                to_discard_pile: discard_pile.clone(),
            }
        ) {
            println!("Deal in message failed to send {e}");
            // TODO: might need to disconnect client here, or return to lobby?
        }

        if agent.turn_id == 0 {
            if let Err(e) = bincode::serialize_into(
                &player.stream, 
                &FromServer::YourTurn
            ) {
                println!("Leave lobby message failed to send {e}");
                // TODO: might need to disconnect client here, or return to lobby?
            }
        }
    }
}

pub fn spawn_tables (
    mut commands: Commands,
    agent_tracker: Res<AgentTracker>,
) {
    let mut map = TableMap::default();

    let starting_deck = new_deck(); //deck_builder.make_cards(105);

    // Make deck table
    let table = ServerTable::new(starting_deck);
    let deck_entity = commands.spawn()
        .insert(table).id();
    map.0.insert(Location::Deck, deck_entity);

    // Make discard table
    let table = commands.spawn()
        .insert(ServerTable::default()).id();
    map.0.insert(Location::DiscardPile, table);

    spawn_player_hand_tables(
        &mut map,
        &mut commands,
        agent_tracker.agents.len()
    );

    commands.insert_resource(GameInfo::new(agent_tracker.agents.len()));
    commands.insert_resource(map);
}

fn spawn_player_hand_tables(
    map: &mut TableMap,
    commands: &mut Commands,
    num_players: usize,
) {
    for player_id in 0..num_players {
        let table = commands.spawn()
            .insert(ServerTable::default()).id();
        map.0.insert(Location::Hand{player_id}, table);
    }
}