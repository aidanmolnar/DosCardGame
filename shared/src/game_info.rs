
pub struct GameInfo {
    num_players: usize,
    current_turn: usize,
    direction: Direction,
}

enum Direction {
    Clockwise, 
    CounterClockwise
}

impl GameInfo {
    pub fn new(num_players: usize) -> Self {
        GameInfo { 
            num_players, 
            current_turn: 0, 
            direction: Direction::Clockwise 
        }
    }

    pub fn switch_direction (&mut self) {
        self.direction = match &self.direction {
            Direction::Clockwise => Direction::CounterClockwise,
            Direction::CounterClockwise => Direction::Clockwise,
        }
    }

    pub fn next_turn (&mut self) -> usize {
        let offset = match &self.direction {
            Direction::Clockwise => 1,
            Direction::CounterClockwise => -1,
        };
        let total = self.current_turn as i32 + offset;
        self.current_turn = (total  % self.num_players as i32) as usize;

        self.current_turn
    }

    pub fn current_turn (&self) -> usize {
        self.current_turn
    }
}