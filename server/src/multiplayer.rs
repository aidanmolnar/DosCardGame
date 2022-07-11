
use bevy::prelude::*;
use std::net::TcpStream;

// Maintains an ordered list of agents
#[derive(Default)]
pub struct AgentTracker {
    pub agents: Vec<Entity>,
}

// A player in the game. Can be a bot or a human
#[derive(Component)]
pub struct Agent {
    pub name: String,
    pub turn_id: u8
}

// A holder for a stream to a human-controlled client
#[derive(Component)]
pub struct NetPlayer {
    pub stream: TcpStream,
}


