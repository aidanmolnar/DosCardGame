use super::GameState;
use super::MultiplayerState;

pub mod assets;
pub mod layout;
pub mod card_building;
pub mod client_actions;
mod setup;
mod server_actions;
mod networking;
mod table;
mod make_tables;
mod transfer_card;
mod arrange_table;
pub mod animations;
mod populate_deck;
mod mouse;

use networking::game_network_system;
use assets::load_assets;
use setup::{add_deck_sprite, add_camera};
use server_actions::dealing::delayed_dealing_system;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_mod_picking::{PickingPlugin,InteractablePickingPlugin, /* PickingEvent */};



pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app

        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        //.add_plugin(card_building::CardBuildingPlugin)
        .add_plugin(animations::AnimationPlugin)

        // On state startup
        .add_enter_system(GameState::InGame, add_deck_sprite)
        .add_enter_system(GameState::InGame, add_camera)
        .add_enter_system(GameState::InGame, make_tables::make_tables_system)

        // On app startup
        .add_startup_system(load_assets)

        .init_resource::<mouse::FocusedCard>()
        .add_system(mouse::focus_system
            .run_in_state(GameState::InGame))
        .add_system(mouse::update_mouse_system
            .run_in_state(GameState::InGame))


        .add_system(arrange_table::update_table_system
            .run_in_state(GameState::InGame))
        .add_system(delayed_dealing_system
            .run_in_state(GameState::InGame))
        .add_system(game_network_system
            .run_in_state(GameState::InGame));
    }

}