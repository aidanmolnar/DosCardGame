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
mod reconnect;

use dos_shared::GameState;

use game::GamePlugin;
use lobby::LobbyPlugin;
use multiplayer::MultiplayerState;
use reconnect::ReconnectPlugin;

use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;
use iyes_loopless::prelude::*;
use postgame::PostGamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Dos!".to_string(),
                width: 1920.,
                height: 1080.,
                resizable: true,
                position: WindowPosition::Centered,
                ..default()
            },
            ..default()
        }))
        // Networking
        .add_plugin(RenetClientPlugin::default())
        .init_resource::<MultiplayerState>()
        // Starting state
        .add_loopless_state(GameState::MainMenu)
        // Dos plugins
        .add_plugin(LobbyPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(PostGamePlugin)
        .add_plugin(ReconnectPlugin)
        .run();
}
