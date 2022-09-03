#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use dos_shared::GameState;

mod lobby;
mod game;
mod postgame;
mod multiplayer;

use game::GamePlugin;
use lobby::LobbyPlugin;
use multiplayer::MultiplayerState;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use postgame::PostGamePlugin;

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

        // Starting state
        .add_loopless_state(GameState::MainMenu)

        // Dos plugins
        .add_plugin(LobbyPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(PostGamePlugin)
        
        .run()
}