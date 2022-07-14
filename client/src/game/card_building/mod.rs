use super::GameState;
use super::assets;

pub mod card_indexing;
pub mod components;
pub mod templates;
mod systems;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct CardBuildingPlugin;

impl Plugin for CardBuildingPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(
            systems::card_blueprint_system
            .run_in_state(GameState::InGame))
        .add_system(systems::pickable_blueprint_system
            .run_in_state(GameState::InGame));
    }
}
