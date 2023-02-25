use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

// This struct stores all game information that is not related to card positions or "call dos" events
// Namely: turn advancement, current turn tracking, turn direction, draw two / draw four stacking
#[derive(Serialize, Deserialize, Debug, Clone, Resource)]
pub struct GameInfo {
    num_players: usize,
    current_turn: usize,
    direction: TurnDirection,
    pub stacked_draws: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum TurnDirection {
    #[default]
    Clockwise,
    CounterClockwise,
}

impl GameInfo {
    #[must_use]
    pub const fn new(num_players: usize) -> Self {
        Self {
            num_players,
            current_turn: 0,
            direction: TurnDirection::Clockwise,
            stacked_draws: 0,
        }
    }

    pub fn switch_direction(&mut self) {
        self.direction = match &self.direction {
            TurnDirection::Clockwise => TurnDirection::CounterClockwise,
            TurnDirection::CounterClockwise => TurnDirection::Clockwise,
        }
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn current_direction(&self) -> &TurnDirection {
        &self.direction
    }

    pub fn next_turn(&mut self) {
        let step = match &self.direction {
            TurnDirection::Clockwise => 1,
            TurnDirection::CounterClockwise => -1,
        };

        let current_turn = isize::try_from(self.current_turn)
            .expect("Player count should not be large enough to wrap");
        let num_players = isize::try_from(self.num_players)
            .expect("Player count should not be large enough to wrap");

        let total = current_turn + step;
        self.current_turn = total.rem_euclid(num_players) as usize; // Loop back around once below zero or at num_players
    }

    pub fn skip_turn(&mut self) {
        self.next_turn();

        if self.num_players > 1 {
            self.next_turn();
        }
    }

    #[must_use]
    pub const fn current_turn(&self) -> usize {
        self.current_turn
    }

    #[must_use]
    pub const fn num_players(&self) -> usize {
        self.num_players
    }
}
