use std::net::TcpStream;

// Maintains an ordered list of agents
#[derive(Default)]
pub struct AgentTracker {
    agents: Vec<Agent>,
    updated: bool
}

struct Agent {
    name: String,
    ty: AgentType
}

#[allow(dead_code)] // Will be used in the future
enum AgentType {
    Player {
        stream: TcpStream, 
        connection_status: ConnectionStatus
    },
    Bot, 
}

#[derive(PartialEq, Eq, Debug)]
enum ConnectionStatus {
    Connected,
    Desynced,
    Disconnected,
}

impl AgentTracker {
    pub fn new_player(&mut self, name: String, stream: TcpStream) {
        self.updated = true;
        self.agents.push(Agent {
            name,
            ty: AgentType::Player{
                stream, 
                connection_status: ConnectionStatus::Connected,
            },
        })
    }

    #[allow(dead_code)] // Will be used in the future
    pub fn new_bot(&mut self, name: String) {
        self.updated = true;
        self.agents.push(Agent {
            name,
            ty: AgentType::Bot,
        })
    }

    pub fn names(&self) -> Vec<String> {
        self.agents.iter().map(
            |agent|
            match &agent.ty {
                AgentType::Player {connection_status,.. } => {
                    if *connection_status == ConnectionStatus::Disconnected {
                        "[DC] ".to_owned() + &agent.name
                    } else {
                        agent.name.clone()
                    }
                },
                AgentType::Bot => "[BOT] ".to_owned() + &agent.name,
            }
        ).collect()
    }

    pub fn num_agents(&self) -> usize {
        self.agents.len()
    }

    pub fn iter_ids_and_streams(&self) -> impl Iterator<Item = (usize, &TcpStream)> {
        self.agents.iter().enumerate().filter_map(|(i,agent)|{
            match &agent.ty {
                AgentType::Player{stream, ..} => Some((i, stream)),
                AgentType::Bot => None,
            }
        })
    }

    pub fn iter_streams(&self) -> impl Iterator<Item = &TcpStream> {
        self.agents.iter().filter_map(|agent| {
            match &agent.ty {
                AgentType::Player{stream, ..} => Some(stream),
                AgentType::Bot => None,
            }
        })
    }

    #[allow(dead_code)] // Will be used in the future
    pub fn remove(&mut self, player: &usize) {
        self.updated = true;
        self.agents.remove(*player);
    }

    pub fn try_get_stream(&self, player: usize) -> Option<&TcpStream> {
        match &self.agents[player].ty {
            AgentType::Player{stream,..} => Some(stream),
            AgentType::Bot => None,
        }
    }

    pub fn disconnect_player(&mut self, player: usize) {
        self.updated = true;
        match &mut self.agents[player].ty {
            AgentType::Player{stream, connection_status,..} => {
                stream.shutdown(std::net::Shutdown::Both).expect("Couldn't close stream!");
                *connection_status = ConnectionStatus::Disconnected;
            },
            AgentType::Bot => panic!("Can't disconnect a bot"),
        }
    }

    pub fn remove_disconnected_players(&mut self) {
        self.updated = true;
        self.agents.retain(
            |agent| 
            match &agent.ty {
                AgentType::Player { connection_status,.. } => *connection_status == ConnectionStatus::Connected,
                AgentType::Bot => true,
            }
        )
    }

    pub fn is_connected(&self, player: usize) -> bool{
        match &self.agents[player].ty {
            AgentType::Player{connection_status,..} => *connection_status == ConnectionStatus::Connected,
            AgentType::Bot => true,
        }
    }

    pub fn is_desynced(&self, player: usize) -> bool{
        match &self.agents[player].ty {
            AgentType::Player{connection_status,..} => *connection_status == ConnectionStatus::Desynced,
            AgentType::Bot => false,
        }
    }

    pub fn is_disconnected(&self, player: usize) -> bool{
        match &self.agents[player].ty {
            AgentType::Player{connection_status,..} => *connection_status == ConnectionStatus::Disconnected,
            AgentType::Bot => false,
        }
    }

    pub fn were_players_updated(&self) -> bool {
        self.updated
    }

    pub fn reset_players_updated(&mut self) {
        self.updated = false;
    }

    pub fn player_id_from_name(&mut self, name: &str) -> Option<usize> {
        for (i,agent) in self.agents.iter().enumerate() {
            if agent.name == name {
                return Some(i)
            }
        }
        None
    }

    pub fn reconnect_player(&mut self, player: usize, new_stream: TcpStream) {
        self.updated = true;
        match &mut self.agents[player].ty {
            AgentType::Player{stream,connection_status,..} => {
                *stream = new_stream;
                *connection_status = ConnectionStatus::Desynced;
            }
            AgentType::Bot => panic!("Can't reconnect bot"),
        }
    }

    pub fn resync_player(&mut self, player: usize) {
        self.updated = true;
        match &mut self.agents[player].ty {
            AgentType::Player{connection_status,..} => {
                *connection_status = ConnectionStatus::Connected;
            }
            AgentType::Bot => panic!("Can't resync bot"),
        }
    }

    pub fn all_connected(&self) -> bool {
        self.agents.iter().all(
            |agent|
            match &agent.ty {
                AgentType::Player {connection_status,.. } => *connection_status == ConnectionStatus::Connected,
                AgentType::Bot => true,
            }
        )
    }

    pub fn all_disconnected(&self) -> bool {
        self.agents.iter().all(
            |agent|
            match &agent.ty {
                AgentType::Player {connection_status,.. } => !(*connection_status == ConnectionStatus::Connected),
                AgentType::Bot => true,
            }
        )
    }
}