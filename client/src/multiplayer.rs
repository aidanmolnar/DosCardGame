
#[derive(Default, Debug)]
pub struct MultiplayerState {
    pub player_names: Vec<String>,
    pub turn_id: usize,
}

impl MultiplayerState {
    pub fn connect(&mut self, player_names: Vec<String>) {
        self.player_names = player_names;
    }

    pub fn disconnect(&mut self) {
        self.player_names = Vec::new();
    }
}