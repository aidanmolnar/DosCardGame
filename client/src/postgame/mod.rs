use dos_shared::GameState;

mod ui;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct PostGamePlugin;

pub struct Victory{pub winner: usize}

impl Plugin for PostGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ui::postgame_ui.run_in_state(GameState::PostGame));
    }
}

