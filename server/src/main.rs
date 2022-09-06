use dos_shared::GameState;

mod connection_listening;
mod multiplayer;
mod lobby;
mod game;
mod reconnect;
mod postgame;

use lobby::LobbyPlugin;
use game::GamePlugin;
use postgame::PostgamePlugin;
use reconnect::ReconnectPlugin;
use connection_listening::ConnectionListeningPlugin;
pub use connection_listening::ConnectionTask;

use bevy::prelude::*;
use iyes_loopless::prelude::*;


fn main() {
    App::new()
        .add_plugins(MinimalPlugins)

        // Starting State
        .add_loopless_state(GameState::MainMenu)

        // Dos Plugins
        .add_plugin(ConnectionListeningPlugin)
        .add_plugin(LobbyPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(ReconnectPlugin)
        .add_plugin(PostgamePlugin)

        .run()
}