#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::needless_pass_by_value)] // Bevy systems require resources to be passed by value
#![allow(clippy::only_used_in_recursion)] // No recursive functions and had false positives

mod lobby;
mod game;
mod postgame;
mod multiplayer;
mod reconnect;
mod connections;

use dos_shared::GameState;

use game::GamePlugin;
use lobby::LobbyPlugin;
use reconnect::ReconnectPlugin;
use multiplayer::MultiplayerState;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use postgame::PostGamePlugin;
use bevy_renet::RenetClientPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Dos!".to_string(),
            width: 1280.,
            height: 720.,
            resizable: true,
            position: WindowPosition::Centered(MonitorSelection::Primary),
            ..default()
        })
        .add_plugins(DefaultPlugins)

        // Networking 
        .add_plugin(RenetClientPlugin)
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