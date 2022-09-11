mod ui;

use dos_shared::GameState;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

// Ui for after game screen
pub struct PostGamePlugin;

// Holds which player won the game
pub struct Victory{pub winner: usize}

impl Plugin for PostGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ui::postgame_ui_system.run_in_state(GameState::PostGame));
    }
}

