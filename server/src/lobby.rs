use super::GameState;
use super::{multiplayer, connection_listening};
use connection_listening::PlayerCountChange;

mod networking;
use networking::{lobby_network_system, handle_playercount_change_system, leave_lobby_system};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(lobby_network_system
            .run_in_state(GameState::MainMenu)
            .after("handle_changes"))

        .add_system(handle_playercount_change_system
            .run_in_state(GameState::MainMenu)
            .run_on_event::<PlayerCountChange>()
            .label("handle_changes"))
        .add_exit_system(GameState::MainMenu, leave_lobby_system);
    }
}