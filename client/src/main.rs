use std::net::{TcpStream};

use bevy::app::AppExit;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_egui::{egui::{self, Color32}, EguiContext, EguiPlugin};

use dos_shared::*;


// https://github.com/IyesGames/iyes_loopless/blob/main/examples/menu.rs
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<UiState>() // TODO: How to integrate this with iyes?? Deallocate once in game?
        .init_resource::<MultiplayerState>()

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

        .add_stage_after(
            CoreStage::Last,
            "very_last",
            SystemStage::single_threaded()
        )
        .add_system_to_stage("very_last", close_event_listener
            .run_on_event::<AppExit>()
            .run_if_resource_exists::<MultiplayerState>()

        )
        
        .run()
}

/// Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    //InGame,
}

const DEFAULT_IP: &str = "localhost:3333";

pub struct UiState {
    ip: String,
    name: String,
    error: &'static str,
    
}

#[derive(Default)]
pub struct MultiplayerState {
    stream: Option<TcpStream>,
    player_names: Vec<String>,
    is_lobby_leader: bool,
}

impl Default for UiState {
    fn default() -> Self {
        UiState{
            ip: DEFAULT_IP.to_string(), 
            name: "".to_string(),
            error: "", 
    }}
}

fn lobby_ui(mut egui_context: ResMut<EguiContext>, mut ui_state: ResMut<UiState>, mut mp_state: ResMut<MultiplayerState>) {
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

            } else {
                ui_state.error = "Connection Failed";
            }
        } else if mp_state.stream.is_some() && ui.button("Disconnect").clicked(){
            disconnect(mp_state.stream.as_ref());

            // Reset state to default
            mp_state.stream = None;
            mp_state.player_names = Vec::new();
            mp_state.is_lobby_leader = false;
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

fn update_lobby(mut mp_state: ResMut<MultiplayerState>) {
    //mp_state.stream.read(buf)

    let stream =
        match &mp_state.stream {
            None => return,
            Some(i) => i,
    };
    
    match bincode::deserialize_from::<&TcpStream, LobbyUpdateServer>(stream) {
        Ok(lobby_update) => {
            println!("{:?}", lobby_update);

            match lobby_update {
                LobbyUpdateServer::CurrentPlayers{player_names} => {
                    mp_state.player_names = player_names;
                }
                LobbyUpdateServer::YouAreLobbyLeader => {
                    mp_state.is_lobby_leader = true;
                }
            }
        },
        Err(e) => {
            handle_error(e);
        }
    }
}

fn send_start_game (stream: Option<&TcpStream>)  {
    if let Some(stream) = stream {
        bincode::serialize_into(stream, &LobbyUpdateClient::StartGame);
    }
    
}


// https://www.reddit.com/r/rust/comments/85ebwk/any_tips_on_handling_multiple_error_types_in_rust/

fn connect(address: &str, name: &str) -> Result<TcpStream, Box<dyn std::error::Error>> {
    match TcpStream::connect(address) {
        Ok(stream) => {

            println!("Successfully connected to server {address}");

            // Send the client info (name)
            bincode::serialize_into(&stream, &LobbyUpdateClient::Connect{name: name.to_string()})?;
     
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


fn close_event_listener(mp_state: ResMut<MultiplayerState>) {
    println!("App Exit Event");

    disconnect(mp_state.stream.as_ref())
}

fn disconnect(stream: Option<&TcpStream>) {

    // unwrap stream
    let stream =
        match stream {
            None => return,
            Some(i) => i,
    };


    bincode::serialize_into(stream, &LobbyUpdateClient::Disconnect{}).expect(" Couldn't send disconnect message");

    if let Err(e) = stream.shutdown(std::net::Shutdown::Both) {
        println!("Exit shutdown error: {:?}", e);
    }
}