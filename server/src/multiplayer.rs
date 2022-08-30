use std::net::TcpStream;

// Maintains an ordered list of agents
#[derive(Default)]
pub struct AgentTracker (Vec<Agent>);

struct Agent {
    name: String,
    ty: AgentType
}

#[allow(dead_code)] // Will be used in the future
enum AgentType {
    Player(TcpStream),
    Bot, 
}

impl AgentTracker {
    pub fn new_player(&mut self, name: String, stream: TcpStream) {
        self.0.push(Agent {
            name,
            ty: AgentType::Player(stream),
        })
    }

    #[allow(dead_code)] // Will be used in the future
    pub fn new_bot(&mut self, name: String) {
        self.0.push(Agent {
            name,
            ty: AgentType::Bot,
        })
    }

    pub fn names(&self) -> Vec<String> {
        self.0.iter().map(|x|x.name.clone()).collect()
    }

    pub fn num_agents(&self) -> usize {
        self.0.len()
    }

    pub fn iter_ids_and_streams(&self) -> impl Iterator<Item = (usize, &TcpStream)> {
        self.0.iter().enumerate().filter_map(|(i,agent)|{
            match &agent.ty {
                AgentType::Player(stream) => Some((i, stream)),
                AgentType::Bot => None,
            }
        })
    }

    pub fn iter_streams(&self) -> impl Iterator<Item = &TcpStream> {
        self.0.iter().filter_map(|agent| {
            match &agent.ty {
                AgentType::Player(stream) => Some(stream),
                AgentType::Bot => None,
            }
        })
    }

    pub fn remove(&mut self, player: usize) {
        self.0.remove(player);
    }

    pub fn try_get_stream(&self, player: usize) -> Option<&TcpStream> {
        match &self.0[player].ty {
            AgentType::Player(stream) => Some(stream),
            AgentType::Bot => None,
        }
    }
}