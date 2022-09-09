use bevy_egui::egui::Ui;
use bevy_renet::renet::RenetClient;
use dos_shared::DEFAULT_IP;
use super::networking::*;
use super::connections::{ConnectionTask, create_connection_task};
use super::MultiplayerState;

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy_egui::{egui::{self, Color32}, EguiContext};

pub struct UiState {
    ip: String,
    name: String,
    error: String,
    status: ConnectionStatus,
}

impl UiState {
    pub fn set_connected(&mut self) {
        self.error = "".to_owned();
        self.status = ConnectionStatus::Connected;
    }
    pub fn set_disconnected(&mut self, error_message: String) {
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
            error: "".to_string(), 
            status: ConnectionStatus::Disconnected,
    }}
}

// A barebones egui for connecting to the game
pub fn lobby_ui(
    mut egui_context: ResMut<EguiContext>, 
    mut ui_state: ResMut<UiState>, 
    mut mp_state: ResMut<MultiplayerState>,
    mut commands: Commands,
    mut renet_client_opt: Option<ResMut<RenetClient>>,
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

        connect_button_ui(
            ui, 
            ui_state.as_mut(), 
            mp_state.as_mut(), 
            &mut commands, 
            renet_client_opt.as_deref_mut()
        );
        
        if !ui_state.error.is_empty() {
            ui.colored_label(Color32::from_rgb(255,0,0), &ui_state.error);
        }

        if !mp_state.player_names.is_empty() && renet_client_opt.is_some() {
            ui.label("Players:");

            for player in &mp_state.player_names {
                ui.label(player);
            }
        }

        if let Some(renet_client) = renet_client_opt.as_mut() {
            if mp_state.turn_id == 0 {
                ui.label("You are the Lobby Leader");
                if ui.button("Start Game").clicked() {
                    send_start_game(renet_client);
                    println!("Start the game");
                }
            }
        }

    });
}

fn connect_button_ui (
    ui: &mut Ui,
    ui_state: &mut UiState,
    mp_state: &mut MultiplayerState,
    commands: &mut Commands,
    mut renet_client_opt: Option<&mut RenetClient>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    match ui_state.status {
        ConnectionStatus::Disconnected => {
            if ui.button("Connect").clicked() {
                let address = ui_state.ip.clone();
                let name = ui_state.name.clone();

                // Spawn a task for connecting to the server if the ip was valid
                if let Ok(socket_address) = address.parse() {
                    let task = thread_pool.spawn(async move {
                        create_connection_task(socket_address, &name)
                    });
                    commands.spawn().insert(ConnectionTask(task));

                    ui_state.status = ConnectionStatus::Connecting;
                } else {
                    ui_state.error = "Failed to parse server address".to_string();
                }
            }    
        },
        ConnectionStatus::Connecting => {
            ui.add(egui::Button::new("Connecting..."));
        },
        ConnectionStatus::Connected => {
            if ui.button("Disconnect").clicked() {
                if let Some(renet_client) = renet_client_opt.as_mut() {
                    renet_client.disconnect();
                    commands.remove_resource::<RenetClient>();
                }
                
                mp_state.disconnect();
                ui_state.status = ConnectionStatus::Disconnected;
            }
        },
    }
}