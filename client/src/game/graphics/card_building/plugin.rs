use super::GameState;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::systems::*;

pub struct CardBuildingPlugin;

impl Plugin for CardBuildingPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(
            card_blueprint_system
            .run_in_state(GameState::InGame))
        .add_system(pickable_blueprint_system
            .run_in_state(GameState::InGame));
    }
}