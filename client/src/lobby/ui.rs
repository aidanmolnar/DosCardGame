use dos_shared::DEFAULT_IP;
use super::networking::*;
use super::connecting::create_connection_task;
use super::MultiplayerState;

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy_egui::{egui::{self, Color32}, EguiContext};


// TODO: break up into sub modules (1 for UI state, 1 for building the lobby ui?)

pub struct UiState {
    ip: String,
    name: String,
    error: &'static str,
    status: ConnectionStatus,
}

impl UiState {
    pub fn set_connected(&mut self) {
        self.error = "";
        self.status = ConnectionStatus::Connected;
    }
    pub fn set_disconnected(&mut self, error_message: &'static str) {
        self.error = error_message;
        self.status = ConnectionStatus::Disconnected;
    }
}

enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
}

impl Default for UiState {
    fn default() -> Self {
        UiState{
            ip: DEFAULT_IP.to_string(), 
            name: "".to_string(),
            error: "", 
            status: ConnectionStatus::Disconnected,
    }}
}

// TODO: break up into smaller functions
pub fn lobby_ui(
    mut egui_context: ResMut<EguiContext>, 
    mut ui_state: ResMut<UiState>, 
    mut mp_state: ResMut<MultiplayerState>,
    mut commands: Commands,
    thread_pool: Res<AsyncComputeTaskPool>,
) {
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

        match ui_state.status {
            ConnectionStatus::Disconnected => {

                if ui.button("Connect").clicked() {

                    let address = ui_state.ip.clone();
                    let name = ui_state.name.clone();

                    let task = thread_pool.spawn(async move {
                        create_connection_task(&address, &name)
                    });
                    ui_state.status = ConnectionStatus::Connecting;
                    commands.spawn().insert(task);
                }
                
            },
            ConnectionStatus::Connecting => {
                ui.add(egui::Button::new("Connecting..."));
            },
            ConnectionStatus::Connected => {
                if ui.button("Disconnect").clicked() {
                    mp_state.set_disconnected();
                    ui_state.status = ConnectionStatus::Disconnected;
                }
            },
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

        if mp_state.stream.is_some() && mp_state.turn_id == 0 {
            ui.label("You are the Lobby Leader");
            if ui.button("Start Game").clicked() {
                send_start_game(mp_state.stream.as_ref());
                println!("Start the game");
            }
        }

    });
}

