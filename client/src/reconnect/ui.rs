use bevy::prelude::*;
use bevy_egui::{EguiContext, egui};

use crate::multiplayer::MultiplayerState;

pub fn reconnect_ui_system(
    mut egui_context: ResMut<EguiContext>, 
    mp_state: Res<MultiplayerState>,
) {
    egui::SidePanel::left("left_panel").show(egui_context.ctx_mut(), |ui| {

        ui.label("A player disconnected!!!");

        if !mp_state.player_names.is_empty() {
            ui.label("Players:");

            for player in &mp_state.player_names {
                ui.label(player);
            }
        }

        if mp_state.turn_id == 0 {
            ui.label("You are lobby leader");
        }
    });
}