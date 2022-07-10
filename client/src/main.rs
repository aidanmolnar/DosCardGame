mod lobby;
mod game;
mod multiplayer;

use game::GamePlugin;
use lobby::LobbyPlugin;
use multiplayer::MultiplayerState;

//use bevy::app::AppExit;
use bevy::prelude::*;

use iyes_loopless::prelude::*;

use bevy_mod_picking::*;

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

        // Starting state
        .add_loopless_state(GameState::MainMenu)

        // Core plugins
        .add_plugins(DefaultPlugins)
        

        // Dos plugins
        .add_plugin(LobbyPlugin)
        .add_plugin(GamePlugin)
            

        //.add_system_to_stage(CoreStage::PostUpdate, print_events.run_in_state(GameState::InGame))
        
        .run()
}

// testing picking/selecting cards
pub fn print_events(mut events: EventReader<PickingEvent>) {

    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => info!("Gee Willikers, it's a click! {:?}", e),
        }
    }
}