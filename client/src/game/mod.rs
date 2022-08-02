use dos_shared::table::TableMap;

use super::GameState;
use super::MultiplayerState;

pub mod assets;
pub mod layout;
pub mod card_indexing;
pub mod client_actions;
mod camera;
mod server_actions;
mod networking;
pub mod table;
pub mod animations;

use assets::load_assets;
use camera::add_camera;
use animations::AnimationPlugin;
use networking::game_network_system;
use server_actions::delayed_dealing_system;
use table::TablePlugin;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_mod_picking::{PickingPlugin,InteractablePickingPlugin, /* PickingEvent */};


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app

        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(TablePlugin)

        // On state startup
        .add_enter_system(GameState::InGame, add_camera)
        
        // On app startup
        .add_startup_system(load_assets)

        .add_system(game_network_system
            .run_in_state(GameState::InGame))

        // TODO: move this
        .add_system(delayed_dealing_system
            .run_in_state(GameState::InGame)
            .run_if_resource_exists::<TableMap>()
        );

    }

}