#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::needless_pass_by_value)] // Bevy systems require resources to be passed by value
#![allow(clippy::only_used_in_recursion)] // No recursive functions and had false positives

mod connections;
mod game;
mod lobby;
mod multiplayer;
mod postgame;

use dos_shared::GameState;

use bevy::prelude::*;
use bevy_renet::RenetServerPlugin;
use iyes_loopless::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        // Starting State
        .add_loopless_state(GameState::MainMenu)
        //Networking
        .add_plugin(RenetServerPlugin::default())
        .init_resource::<multiplayer::MultiplayerState>()
        // Dos Plugins
        .add_plugin(connections::ConnectionListeningPlugin)
        .add_plugin(lobby::LobbyPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(postgame::PostgamePlugin)
        .run();
}
