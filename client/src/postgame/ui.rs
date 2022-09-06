use dos_shared::GameState;

use bevy::prelude::*;
use bevy_egui::{EguiContext, egui};
use iyes_loopless::state::NextState;

use crate::multiplayer::MultiplayerState;

use super::Victory;

pub fn postgame_ui(
    mut egui_context: ResMut<EguiContext>, 
    mut commands: Commands,
    victory: Res<Victory>,
    mp_state: Res<MultiplayerState>,
) {
    egui::SidePanel::left("left_panel").show(egui_context.ctx_mut(), |ui| {

        ui.label(format!("{} won the game!", mp_state.player_names[victory.winner])); // TODO: Make this robust to errors with multiplayer state. Crashes client if server crashes currently...
        if ui.button("Return to Main Menu").clicked() {
            commands.insert_resource(NextState(GameState::MainMenu));
        }
    });
}