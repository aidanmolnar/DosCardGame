use std::net::{TcpStream};
use std::io::{Read, Write};
use std::io;


use bevy::app::AppExit;
use bevy::ecs::event::Events;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use dos_shared::*;


// https://github.com/IyesGames/iyes_loopless/blob/main/examples/menu.rs
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<UiState>() // TODO: How to integrate this with iyes?? Deallocate once in game??

        .add_system(close_event_listener
            .run_if_resource_exists::<MultiplayerState>())


        .add_loopless_state(GameState::MainMenu)

        // Main menu systems
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(lobby_ui)
                .with_system(update_lobby
                    .run_if_resource_exists::<MultiplayerState>())
                //
                .into()
        )
        
        .run()
}

/// Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    InGame,
}

const DEFAULT_IP: &str = "localhost:3333";

pub struct UiState {
    pub ip: String,
    pub name: String,
    pub error: &'static str,
    pub players: Vec<String>,
}

pub struct MultiplayerState {
    stream: TcpStream,
}

impl Default for UiState {
    fn default() -> Self {
        UiState{
            ip: DEFAULT_IP.to_string(), 
            name: "".to_string(), 
            error: "", 
            players: Vec::new()
    }}
}

fn lobby_ui(mut egui_context: ResMut<EguiContext>, mut ui_state: ResMut<UiState>, mut commands: Commands) {
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
        
        if ui.button("Connect").clicked() {
            if let Ok(stream) = connect(&ui_state.ip, &ui_state.name) {
                commands.insert_resource(MultiplayerState{stream});
                //commands.insert_resource(NextState(GameState::InGame));
            } else {
                ui_state.error = "Connection Failed";
            }
        };

        if !ui_state.error.is_empty() {
            ui.label(ui_state.error);
        }

        if !ui_state.players.is_empty() {
            for player in &ui_state.players {
                ui.label(player);
            }
        }
    });
}

fn update_lobby(mut ui_state: ResMut<UiState>, mp_state: Res<MultiplayerState>) {


    //mp_state.stream.read(buf)

    match bincode::deserialize_from::<&TcpStream, LobbyUpdate>(&mp_state.stream) {
        Ok(lobby_update) => {
            println!("{:?}", lobby_update);

            match lobby_update {
                LobbyUpdate::PlayerCount{players} => {
                    ui_state.players = players;
                }
            }
            

        },
        Err(e) => {
            handle_error(e);
        }
    }
}

fn handle_error(e: Box<bincode::ErrorKind>) {
    match *e {
        bincode::ErrorKind::Io(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
        _ => {
            println!("Failed to receive data: {}", e);
        }
    }
}

// https://www.reddit.com/r/rust/comments/85ebwk/any_tips_on_handling_multiple_error_types_in_rust/

fn connect(address: &str, name: &str) -> Result<TcpStream, Box<dyn std::error::Error>> {
    match TcpStream::connect(address) {
        Ok(stream) => {

            println!("Successfully connected to server {address}");

            // Send the client info (name)
            bincode::serialize_into(&stream, &ClientConnect{name: name.to_string()})?;
     
            //TODO: Receive the state update here?

            stream.set_nonblocking(true)?;
 
            Ok(stream)
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
            Err(Box::new(e))
        }
    }

}

fn close_event_listener(mut events: EventReader<AppExit>, mp_state: Res<MultiplayerState>) {

    for event in events.iter() {
        println!("EVENT: {:?}", event);
        if let Err(e) = mp_state.stream.shutdown(std::net::Shutdown::Both) {
            println!("{:?}", e);
        }
    }
}
