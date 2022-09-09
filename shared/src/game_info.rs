use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameInfo {
    num_players: usize,
    current_turn: usize,
    direction: Direction,
    pub stacked_draws: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Direction {
    Clockwise, 
    CounterClockwise
}

impl GameInfo {
    #[must_use]
    pub const fn new(num_players: usize) -> Self {
        Self { 
            num_players, 
            current_turn: 0, 
            direction: Direction::Clockwise,
            stacked_draws: 0,
        }
    }

    pub fn switch_direction(&mut self) {
        self.direction = match &self.direction {
            Direction::Clockwise => Direction::CounterClockwise,
            Direction::CounterClockwise => Direction::Clockwise,
        }
    }

    pub fn next_turn(&mut self)  {
        let offset = match &self.direction {
            Direction::Clockwise => 1,
            Direction::CounterClockwise => -1,
        };

        let current_turn = isize::try_from(self.current_turn).expect("Player count should not be large enough to wrap");
        let num_players  = isize::try_from(self.num_players ).expect("Player count should not be large enough to wrap");

        let total = current_turn + offset;
        self.current_turn = total.rem_euclid(num_players) as usize;
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