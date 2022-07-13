use super::GameState;
use super::MultiplayerState;

mod graphics;
pub mod targeting;
mod game_manager;
mod networking;
mod dealing;
pub mod components;
mod mouse_over;

use graphics::GraphicsPlugin;
use networking::game_network_system;
use dealing::delayed_dealing_system;
use game_manager::{setup_game_manager, GameManager};
use game_manager::run_animations;
use game_manager::card_animation_system;
use mouse_over::card_focus_system;
use mouse_over::FocusedCard;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_mod_picking::{PickingPlugin,InteractablePickingPlugin,PickingEvent};


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

        .add_system(run_animations
            .run_in_state(GameState::InGame)
        )

        .add_plugin(GraphicsPlugin)

        .add_enter_system(GameState::InGame, 
            setup_game_manager)

        // Any system that can spawn a card needs to run before the card_animation_system

        .add_system_to_stage(
            CoreStage::PreUpdate,
            card_animation_system
            .run_in_state(GameState::InGame))
        .add_system_to_stage(
            CoreStage::Update,
            card_focus_system
            .run_on_event::<PickingEvent>()
            .run_in_state(GameState::InGame))
    
        .init_resource::<GameManager>()
        .init_resource::<FocusedCard>();
    }
}