mod networking;

use super::{GameState, multiplayer};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(networking::network_system
            .run_in_state(GameState::MainMenu)
        )

        .add_exit_system(
            GameState::MainMenu, networking::leave_system
        );
    }
}