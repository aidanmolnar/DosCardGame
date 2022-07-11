mod lobby;
mod game;
mod multiplayer;

use game::GamePlugin;
use lobby::LobbyPlugin;
use multiplayer::MultiplayerState;

//use bevy::app::AppExit;
use bevy::prelude::*;

use iyes_loopless::prelude::*;

// TODO: Move to shared
/// Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Dos!".to_string(),
            width: 1920.,
            height: 1080.,
            resizable: true,
            position: Some(Vec2::ZERO),
            ..default()
        })
        .add_plugins(DefaultPlugins)

        // Starting state
        .add_loopless_state(GameState::MainMenu)

        // Dos plugins
        .add_plugin(LobbyPlugin)
        .add_plugin(GamePlugin)
        
        .run()
}