use dos_shared::GameState;

mod lobby;
mod game;
mod postgame;
mod multiplayer;
mod reconnect;

use game::GamePlugin;
use lobby::LobbyPlugin;
use reconnect::ReconnectPlugin;
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
        .add_plugin(ReconnectPlugin)
        
        .run()
}