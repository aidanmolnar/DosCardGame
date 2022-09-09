
// Maintains an ordered list of agents
#[derive(Default)]
pub struct MultiplayerState {
    agents: Vec<Agent>,
}

struct Agent {
    name: String,
    ty: AgentType
}

#[allow(dead_code)] // Will be used in the future
enum AgentType {
    Player {
        renet_id: u64,
        status: PlayerStatus
    },
    Bot, 
}

#[derive(PartialEq, Eq, Debug)]
enum PlayerStatus {
    Ready,
    SendState,
    Disconnected,
}

impl MultiplayerState {
    pub fn new_player(
        &mut self,
        name: String, 
        renet_id: u64
    ) {
        self.agents.push(Agent {
            name,
            ty: AgentType::Player{
                renet_id,
                status: PlayerStatus::Ready,
            },
        })
    }

    #[allow(dead_code)] // Will be used in the future
    pub fn new_bot(&mut self, name: String) {

        self.agents.push(Agent {
            name,
            ty: AgentType::Bot,
        })
    }

    pub fn disconnect_player(
        &mut self,
        renet_id: u64,
    ) {
        for agent in &mut self.agents {
            match &mut agent.ty {
                AgentType::Player { renet_id: this_id, status } =>  {
                    if *this_id == renet_id {
                        *status = PlayerStatus::Disconnected;
                    }
                },
                AgentType::Bot => {},
            }
        }
    }

    pub fn player_from_renet_id(&self, renet_id: u64) -> usize {
        for (turn_id, agent) in self.agents.iter().enumerate() {
            match &agent.ty {
                AgentType::Player { renet_id: this_id, .. } =>  {
                    if *this_id == renet_id {
                        return turn_id
                    }
                },
                AgentType::Bot => {},
            }
        }

        panic!("Player not found");
    }

    pub fn renet_id_from_player(&self, player: usize) -> u64 {
        let agent = &self.agents[player];

        match agent.ty {
            AgentType::Player { renet_id, .. } => renet_id,
            AgentType::Bot => panic!("Bots don't have renet ids"),
        }
    }

    pub fn iter_players(&self) -> impl Iterator<Item = (usize, u64)> + '_ {
        self.agents.iter().enumerate().filter_map(
            |(turn_id, agent)|
            match &agent.ty {
                AgentType::Player {status,renet_id} => {
                    if *status == PlayerStatus::Disconnected {
                        None
                    } else {
                        Some((turn_id, *renet_id))
                    }
                },
                AgentType::Bot => None,
            }
        )
    }

    pub fn names(&self) -> Vec<String> {
        self.agents.iter().map(
            |agent|
            match &agent.ty {
                AgentType::Player {status,..} => {
                    if *status == PlayerStatus::Disconnected {
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

    #[allow(dead_code)] // Will be used in the future
    pub fn remove(&mut self, player: &usize) {
        self.agents.remove(*player);
    }

    pub fn remove_disconnected_players(&mut self) {
        self.agents.retain(
            |agent| 
            match &agent.ty {
                AgentType::Player { status,.. } => !(*status == PlayerStatus::Disconnected),
                AgentType::Bot => true,
            }
        )
    }


    pub fn is_disconnected(&self, player: usize) -> bool{
        match &self.agents[player].ty {
            AgentType::Player{status,..} => *status == PlayerStatus::Disconnected,
            AgentType::Bot => false,
        }
    }

    pub fn player_from_name(&self, name: &str) -> Option<usize> {
        for (i,agent) in self.agents.iter().enumerate() {
            if agent.name == name {
                return Some(i)
            }
        }
        None
    }

    pub fn reconnect_player(&mut self, player: usize, new_renet_id: u64) {
        match &mut self.agents[player].ty {
            AgentType::Player{renet_id ,status,..} => {
                *renet_id = new_renet_id;
                *status = PlayerStatus::SendState;
            }
            AgentType::Bot => panic!("Can't reconnect bot"),
        }
    }

    // Iterate over players that are missing game info because they just reconnected.
    // Sets them to ready during iteration.
    pub fn send_state_to_desynced_players(&mut self) -> impl Iterator<Item = (usize, u64)> + '_ {
        self.agents.iter_mut().enumerate().filter_map(
            |(turn_id, agent)|
            match &mut agent.ty {
                AgentType::Player {status,renet_id} => {
                    if *status == PlayerStatus::SendState {
                        *status = PlayerStatus::Ready;
                        Some((turn_id, *renet_id))
                    } else {
                        None
                        
                    }
                },
                AgentType::Bot => None,
            }
        )
    }

    pub fn all_ready(&self) -> bool {
        self.agents.iter().all(
            |agent|
            match &agent.ty {
                AgentType::Player {status,.. } => *status == PlayerStatus::Ready,
                AgentType::Bot => true,
            }
        )
    }

    pub fn all_disconnected(&self) -> bool {
        self.agents.iter().all(
            |agent|
            match &agent.ty {
                AgentType::Player {status,.. } => *status == PlayerStatus::Disconnected,
                AgentType::Bot => true,
            }
        )
    }
}