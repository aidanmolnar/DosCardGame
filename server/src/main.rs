mod connection_listening;
mod multiplayer;
mod lobby;
mod game;

use lobby::LobbyPlugin;
use game::GamePlugin;
use connection_listening::ConnectionListeningPlugin;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

// Move to shared...
/// Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)

        // Starting State
        .add_loopless_state(GameState::MainMenu)

        // Dos Plugins
        .add_plugin(ConnectionListeningPlugin)
        .add_plugin(LobbyPlugin)
        .add_plugin(GamePlugin)

        .run()
}