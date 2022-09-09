use dos_shared::cards::Card;

use crate::multiplayer::MultiplayerState;

use bevy::prelude::*;

pub struct ServerSyncer {
    cards: Vec<Vec<Card>>, 
    conditions: Vec<bool>,
}

impl ServerSyncer {
    fn new(num_players: usize) -> Self {
        let mut players = Vec::with_capacity(num_players);
        for _ in 0..num_players {
            players.push(Vec::new())
        }
        ServerSyncer {
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
        self.conditions.push(condition)
    }

    pub fn take_conditions(&mut self) -> Vec<bool> {
        std::mem::take(&mut self.conditions)
    }
}

pub fn setup_syncer(
    mp_state: Res<MultiplayerState>,
    mut commands: Commands,
) {
    commands.insert_resource(ServerSyncer::new(mp_state.num_agents()))
}