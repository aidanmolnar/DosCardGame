
use super::DEFAULT_IP;
use super::lobby_network::*;


use bevy::prelude::*;
use bevy_egui::{egui::{self, Color32}, EguiContext};

pub struct UiState {
    ip: String,
    name: String,
    error: &'static str,
}

impl Default for UiState {
    fn default() -> Self {
        UiState{
            ip: DEFAULT_IP.to_string(), 
            name: "".to_string(),
            error: "", 
    }}
}

pub fn lobby_ui(mut egui_context: ResMut<EguiContext>, mut ui_state: ResMut<UiState>, mut mp_state: ResMut<MultiplayerState>) {
    egui::SidePanel::left("left_panel").show(
        egui_context.ctx_mut(), |ui| {

        ui.label("Lobby");

        ui.horizontal(|ui| {
            ui.label("Server Address: ");
            ui.text_edit_singleline(&mut ui_state.ip);
        });

        ui.horizontal(|ui| {
            ui.label("Name: ");
            ui.text_edit_singleline(&mut ui_state.name);
        });
        
        if mp_state.stream.is_none() && ui.button("Connect").clicked() {
            if let Ok(stream) = connect(&ui_state.ip, &ui_state.name) {
                mp_state.stream = Some(stream);
                //commands.insert_resource(NextState(GameState::InGame));
                ui_state.error = "";
            } else {
                ui_state.error = "Connection Failed";
            }
        } else if mp_state.stream.is_some() && ui.button("Disconnect").clicked(){
            disconnect(&mut mp_state);
        }
        
        if !ui_state.error.is_empty() {
            ui.colored_label(Color32::from_rgb(255,0,0), ui_state.error);
        }

        if !mp_state.player_names.is_empty() {
            ui.label("Players:");

            for player in &mp_state.player_names {
                ui.label(player);
            }
        }

        if mp_state.is_lobby_leader {
            ui.label("You are the Lobby Leader");
            if ui.button("Start Game").clicked() {
                send_start_game(mp_state.stream.as_ref());
                println!("Start the game");
            }
        }

    });
}