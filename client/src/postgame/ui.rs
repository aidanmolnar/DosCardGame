use dos_shared::GameState;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::state::NextState;

use super::Victory;

// Barebones egui for when the game is over
pub fn postgame_ui_system(
    mut egui_context: ResMut<EguiContext>,
    mut commands: Commands,
    victory: Res<Victory>,
) {
    egui::SidePanel::left("left_panel").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("{} won the game!", victory.winner));

        if ui.button("Return to Main Menu").clicked() {
            commands.insert_resource(NextState(GameState::MainMenu));
        }
    });
}
