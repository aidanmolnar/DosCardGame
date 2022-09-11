use dos_shared::GameState;

use bevy::prelude::*;
use bevy_egui::{EguiContext, egui};
use iyes_loopless::state::NextState;

use crate::multiplayer::MultiplayerState;

use super::Victory;

// Barebones egui for when the game is over
pub fn postgame_ui_system(
    mut egui_context: ResMut<EguiContext>, 
    mut commands: Commands,
    victory: Res<Victory>,
    mp_state: Res<MultiplayerState>,
) {
    egui::SidePanel::left("left_panel").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("{} won the game!", mp_state.player_names[victory.winner])); 

        if ui.button("Return to Main Menu").clicked() {
            commands.insert_resource(NextState(GameState::MainMenu));
        }
    });
}