use dos_shared::cards::Card;

use bevy::prelude::*;

use crate::multiplayer::AgentTracker;


pub struct ServerSyncer {
    cards: Vec<Vec<Card>>, 
    condition_counter: usize
}

impl ServerSyncer {
    fn new(num_players: usize) -> Self {
        let mut players = Vec::with_capacity(num_players);
        for _ in 0..num_players {
            players.push(Vec::new())
        }
        ServerSyncer{
            cards: players, 
            condition_counter: 0,
        }
    }

    pub (crate) fn add(&mut self, player: usize, card: Card) {
        self.cards[player].push(card);
    }

    pub fn take_player(&mut self, player: usize) -> Vec<Card>{
        std::mem::take(&mut self.cards[player])
    }

    pub (crate) fn increment_condition_counter(&mut self) {
        self.condition_counter += 1;
    }

    pub fn take_condition_counter(&mut self) -> usize {
        std::mem::take(&mut self.condition_counter)
    }
}

pub fn setup_syncer(
    agent_tracker: Res<AgentTracker>,
    mut commands: Commands,
) {
    commands.insert_resource(ServerSyncer::new(agent_tracker.agents.len()))
}