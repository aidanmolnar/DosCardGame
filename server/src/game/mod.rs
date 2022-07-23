use super::GameState;
use super::multiplayer;

mod networking;
use networking::enter_game_system;
mod table;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_enter_system(GameState::InGame, enter_game_system);
    }
}