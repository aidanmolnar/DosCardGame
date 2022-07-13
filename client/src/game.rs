use super::GameState;
use super::MultiplayerState;

mod graphics;
pub mod targeting;
mod game_manager;
mod networking;
mod dealing;

use graphics::GraphicsPlugin;
use networking::game_network_system;
use dealing::delayed_dealing_system;
use game_manager::{setup_game_manager, GameManager};

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
        .add_system_to_stage(CoreStage::Update, delayed_dealing_system
            .run_in_state(GameState::InGame)
        )

        .add_plugin(GraphicsPlugin)

        .add_enter_system(GameState::InGame, 
            setup_game_manager)


        .init_resource::<GameManager>();
    }
}