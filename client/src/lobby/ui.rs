use dos_shared::net_config::{DEFAULT_IP, DEFAULT_PORT};

use crate::connections::new_renet_client;

use super::{networking::send_start_game, MultiplayerState};

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, TextEdit},
    EguiContext,
};
use bevy_renet::renet::RenetClient;

// Lobby ui information
pub struct UiState {
    ip: String,
    name: String,
    error: String, // For displaying connection errors
}

impl UiState {
    pub fn set_connected(&mut self) {
        self.error = String::new();
    }
    pub fn set_disconnected(&mut self, error_message: String) {
        self.error = error_message;
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            ip: format!("{DEFAULT_IP}:{DEFAULT_PORT}"),
            name: String::new(),
            error: String::new(),
        }
    }
}

// A barebones egui for connecting to the game
pub fn lobby_ui_system(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut mp_state: ResMut<MultiplayerState>,
    mut commands: Commands,
    mut renet_client_opt: Option<ResMut<RenetClient>>,
) {
    egui::SidePanel::left("left_panel")
        .min_width(400.)
        .max_width(400.)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Lobby");

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Server Address: ");
                TextEdit::singleline(&mut ui_state.ip)
                    .desired_width(f32::INFINITY)
                    .show(ui);
            });

            ui.horizontal(|ui| {
                ui.label("Name: ");
                TextEdit::singleline(&mut ui_state.name)
                    .desired_width(f32::INFINITY)
                    .show(ui);
            });

            connect_button_ui(
                ui,
                ui_state.as_mut(),
                mp_state.as_mut(),
                &mut commands,
                renet_client_opt.as_deref_mut(),
            );

            if !ui_state.error.is_empty() {
                ui.colored_label(Color32::from_rgb(255, 0, 0), &ui_state.error);
            }

            if !mp_state.player_names.is_empty() && renet_client_opt.is_some() {
                ui.separator();
                ui.heading("Players:");
                egui::Grid::new("players").show(ui, |ui| {
                    for player in &mp_state.player_names {
                        ui.label(player);
                        ui.end_row();
                    }
                });
            }

            if let Some(renet_client) = renet_client_opt.as_mut() {
                if renet_client.is_connected() && mp_state.turn_id == 0 {
                    ui.separator();
                    ui.label("You are the Lobby Leader");
                    if ui.button("Start Game").clicked() {
                        send_start_game(renet_client);
                        println!("Start the game");
                    }
                }
            }
        });
}

// Create a button for connecting to the server. Also tracks connection progress
fn connect_button_ui(
    ui: &mut egui::Ui,
    ui_state: &mut UiState,
    mp_state: &mut MultiplayerState,
    commands: &mut Commands,
    renet_client_opt: Option<&mut RenetClient>,
) {
    if let Some(renet_client) = renet_client_opt {
        if !renet_client.is_connected() {
            ui.add(egui::Button::new("Connecting..."));
        } else if ui.button("Disconnect").clicked() {
            renet_client.disconnect();
            mp_state.disconnect();
            commands.remove_resource::<RenetClient>();
        }
    } else if ui.button("Connect").clicked() {
        let address = ui_state.ip.clone();
        let name = ui_state.name.clone();

        // Check name is valid
        if name.len() > 20 {
            ui_state.error = "Name is too long".to_string();
            return;
        }

        if name.is_empty() {
            ui_state.error = "Name is too short".to_string();
            return;
        }

        // Check address is valid
        if let Ok(socket_address) = address.parse() {
            // Attempt to connect a renet client to server
            match new_renet_client(socket_address, &name) {
                Ok(client) => {
                    commands.insert_resource(client);
                    ui_state.set_connected();
                }
                Err(e) => {
                    println!("{e}");
                    ui_state.set_disconnected(format!("Connection Failed: {e}"));
                }
            }
        } else {
            ui_state.error = "Failed to parse server address".to_string();
        }
    }
}
