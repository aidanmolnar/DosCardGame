use self::client_actions::WildCardPlugin;
use self::client_actions::play_card::play_card_system;
use self::table::TablePlugin;

use super::GameState;
use super::MultiplayerState;

pub mod assets;
pub mod layout;
pub mod card_indexing;
pub mod client_actions;
mod camera;
mod networking;
pub mod table;
pub mod animations;

use assets::load_assets;
use camera::add_camera;
use animations::AnimationPlugin;
use networking::game_network_system;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_mod_picking::{PickingPlugin,InteractablePickingPlugin, PickingEvent };


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app

        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(TablePlugin)
        .add_plugin(WildCardPlugin)

        // On state startup
        .add_enter_system(GameState::InGame, add_camera)
        
        
        // On app startup
        .add_startup_system(load_assets)

        .add_system(game_network_system
            .run_in_state(GameState::InGame))

        .add_system(play_card_system
            .run_in_state(GameState::InGame)
            .run_on_event::<PickingEvent>());

    }

}