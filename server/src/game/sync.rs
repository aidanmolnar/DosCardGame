use dos_shared::cards::Card;

use crate::multiplayer::MultiplayerState;

use bevy::prelude::*;

// Stores information that clients will need to reproduce game state that they lack because of visibility rules
pub struct ServerSyncer {
    cards: 
        Vec< // Unique list for each player
            Vec<Card> // Cards that will become visible to player after update
        >,
    conditions: Vec<bool>, // Game logic conditions that depend on cards not visible to player. Shared list for all players
}

impl ServerSyncer {
    fn new(num_players: usize) -> Self {
        let mut players = Vec::with_capacity(num_players);
        for _ in 0..num_players {
            players.push(Vec::new());
        }
        Self {
            cards: players, 
            conditions: Vec::new(),
        }
    }

    pub fn add_card(&mut self, player: usize, card: Card) {
        self.cards[player].push(card);
    }

    pub fn take_player_cards(&mut self, player: usize) -> Vec<Card>{
        std::mem::take(&mut self.cards[player])
    }

    pub fn add_condition(&mut self, condition: bool) {
        self.conditions.push(condition);
    }

    pub fn take_conditions(&mut self) -> Vec<bool> {
        std::mem::take(&mut self.conditions)
    }
}

pub fn setup_syncer(
    mp_state: Res<MultiplayerState>,
    mut commands: Commands,
) {
    commands.insert_resource(
        ServerSyncer::new(mp_state.num_agents())
    );
}