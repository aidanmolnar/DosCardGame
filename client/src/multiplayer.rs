use bevy::prelude::Resource;

// Holds lobby state
#[derive(Default, Debug, Resource)]
pub struct MultiplayerState {
    pub player_names: Vec<String>,
    pub turn_id: usize, // Zero is lobby leader
}

impl MultiplayerState {
    pub fn connect(&mut self, player_names: Vec<String>) {
        self.player_names = player_names;
    }

    pub fn disconnect(&mut self) {
        self.player_names = Vec::new();
    }
}
