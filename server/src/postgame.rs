use dos_shared::GameState;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct PostgamePlugin;

impl Plugin for PostgamePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::PostGame, 
            |mut commands: Commands| {
                commands.insert_resource(NextState(GameState::MainMenu))
            }
        );
            
    }
}