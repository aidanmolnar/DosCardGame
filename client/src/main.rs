use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

use::bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<UiState>()
        .add_system(ui_example)
        .run()
}

const DEFAULT_IP: &str = "localhost:3333";

pub struct UiState {
    pub ip: String,
}

impl Default for UiState {
    fn default() -> Self {UiState{ip: DEFAULT_IP.to_string()} }
}

fn ui_example(mut egui_context: ResMut<EguiContext>, mut ui_state: ResMut<UiState>) {

    egui::SidePanel::left("left_panel").show(
        egui_context.ctx_mut(), |ui| {

        ui.label("Lobby");

        ui.horizontal(|ui| {
            ui.label("Server Address: ");
            ui.text_edit_singleline(&mut ui_state.ip);
        });
        
        if ui.button("Connect").clicked() {
            connect(&ui_state.ip);
        };

    });

}


fn connect(address: &str) {
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let msg = b"Hello!";

            stream.write_all(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}