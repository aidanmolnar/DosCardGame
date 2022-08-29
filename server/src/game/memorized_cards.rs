use dos_shared::cards::Card;

use bevy::prelude::*;

use crate::multiplayer::AgentTracker;


pub struct MemorizedCards (Vec<Vec<Card>>, usize);

impl MemorizedCards {
    fn new(num_players: usize) -> Self {
        let mut players = Vec::with_capacity(num_players);
        for _ in 0..num_players {
            players.push(Vec::new())
        }
        MemorizedCards(players, 0)
    }

    pub (crate) fn add(&mut self, player: usize, card: Card) {
        self.0[player].push(card);
    }

    pub fn take_player(&mut self, player: usize) -> Vec<Card>{
        std::mem::take(&mut self.0[player])
    }

    pub (crate) fn increment_condition_counter(&mut self) {
        self.1 += 1;
    }

    pub fn take_condition_counter(&mut self) -> usize {
        std::mem::take(&mut self.1)
    }
}

pub fn setup_memorized_cards(
    agent_tracker: Res<AgentTracker>,
    mut commands: Commands,
) {
    commands.insert_resource(MemorizedCards::new(agent_tracker.agents.len()))
}