use dos_shared::GameState;

mod connections;
mod multiplayer;
mod lobby;
mod game;
mod postgame;

use lobby::LobbyPlugin;
use game::GamePlugin;
use multiplayer::MultiplayerState;
use postgame::PostgamePlugin;
use connections::ConnectionListeningPlugin;


use bevy::prelude::*;
use bevy_renet::RenetServerPlugin;
use iyes_loopless::prelude::*;


fn main() {
    App::new()
        .add_plugins(MinimalPlugins)

        // Starting State
        .add_loopless_state(GameState::MainMenu)
        
        //Networking
        .add_plugin(RenetServerPlugin)
        .init_resource::<MultiplayerState>()

        // Dos Plugins
        .add_plugin(ConnectionListeningPlugin)
        .add_plugin(LobbyPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(PostgamePlugin)

        .run()
}