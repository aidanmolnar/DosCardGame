use dos_shared::GameState;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

// Immediately moves the server back to the main menu state once game is over
pub struct PostgamePlugin;

impl Plugin for PostgamePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::PostGame, 
            |mut commands: Commands| {
                commands.insert_resource(NextState(GameState::MainMenu));
            }
        );
            
    }
}