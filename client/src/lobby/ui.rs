use dos_shared::DEFAULT_IP;
use super::networking::*;
use super::MultiplayerState;


use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy_egui::{egui::{self, Color32}, EguiContext};

use std::net::TcpStream;
use std::io;
use futures_lite::future;

pub struct UiState {
    ip: String,
    name: String,
    error: &'static str,
    status: ConnectionStatus,
}

enum ConnectionStatus {
    Unconnected,
    Connecting,
    Connected,
}

impl Default for UiState {
    fn default() -> Self {
        UiState{
            ip: DEFAULT_IP.to_string(), 
            name: "".to_string(),
            error: "", 
            status: ConnectionStatus::Unconnected,
    }}
}

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
            ConnectionStatus::Unconnected => {

                if ui.button("Connect").clicked() {

                    let address = ui_state.ip.clone();
                    let name = ui_state.name.clone();

                    let task = thread_pool.spawn(async move {
                        connect(&address, &name)
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
                    disconnect(&mut mp_state);
                    ui_state.status = ConnectionStatus::Unconnected;
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

pub fn handle_connection_task(
    mut transform_tasks: Query<(Entity, &mut Task<Result<TcpStream, io::Error>>)>,
    mut commands: Commands,
    mut mp_state: ResMut<MultiplayerState>,
    mut ui_state: ResMut<UiState>,
) {
    for (entity, mut task) in transform_tasks.iter_mut() {

        if let Some(connection_response) = future::block_on(future::poll_once(&mut *task)) {

            match connection_response {
                Ok(stream) => {
                    mp_state.stream = Some(stream);
                    ui_state.error = "";
                    ui_state.status = ConnectionStatus::Connected;
                }
                Err(e) => {
                    println!("{e}");
                    ui_state.error = "Connection Failed";
                    ui_state.status = ConnectionStatus::Unconnected;
                }
            }

            commands.entity(entity).despawn();
        }
    }
}