use super::GameState;
use super::MultiplayerState;

mod graphics;
mod card_tracker;
mod networking;
mod dealing;

use graphics::GraphicsPlugin;
use networking::game_network_system;
use dealing::delayed_dealing_system;
use card_tracker::CardTracker;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_mod_picking::{PickingPlugin,InteractablePickingPlugin};


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)

        .add_system(game_network_system
            .run_in_state(GameState::InGame))
        .add_system(delayed_dealing_system
            .run_in_state(GameState::InGame)
            .label("dealing") // TODO: clean up this dependency
        )

        .add_plugin(GraphicsPlugin)
       
        
        
        .init_resource::<CardTracker>();
    }
}