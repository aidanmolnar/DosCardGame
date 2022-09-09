use bevy_renet::renet::RenetClient;
use dos_shared::channel_config::GAME_CHANNEL_ID;
use dos_shared::dos_game::DosGame;
use dos_shared::messages::game::*;
use dos_shared::{DECK_SIZE, GameState};
use iyes_loopless::state::NextState;

use super::client_game::ClientGame;
use super::call_dos::CallDos;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
pub struct GameNetworkManager<'w, 's> {
    pub commands: Commands<'w,'s>,
    pub game: ClientGame<'w,'s>,
    renet_client: ResMut<'w, RenetClient>,
}

// Recieves and handles messages from the server
pub fn game_network_system(
    mut network_manager: GameNetworkManager,
) {
    if !network_manager.renet_client.is_connected() {
        network_manager.game.mp_state.disconnect();
        network_manager.commands.remove_resource::<RenetClient>();

        network_manager.commands.insert_resource(NextState(GameState::MainMenu)); 
        return;
    }

    while let Some(message) = network_manager.renet_client.receive_message(GAME_CHANNEL_ID) {
        let update = bincode::deserialize::<FromServer>(&message)
        .expect("Failed to deserialize message!");

        network_manager.handle_update(update);
    }
}

impl<'w,'s> GameNetworkManager<'w,'s> {
    fn handle_update(&mut self, game_update: FromServer) {

        let action = game_update.action.clone();
        self.game.syncer.enque_all(game_update);

        match action {
            GameAction::DealIn => {
                 self.game.deal_starting_cards(DECK_SIZE)
            },
            GameAction::PlayCard(card) => {
                self.game.play_card(&card);
            },
            GameAction::DrawCards => {
                self.game.draw_cards();
            },
            GameAction::KeepStaging => {
                self.game.keep_last_drawn_card();
            },
            GameAction::DiscardWildColor(color) => {
                self.game.declare_wildcard_color(&color);
            },
            GameAction::CallDos(call_dos_info) => {
                let info = call_dos_info.expect("Server always sends caller");
                if info.player != info.caller {
                    self.game.punish_missed_dos(info.player);
                } else {
                    //TODO: Game function to spawn effects?
                }
                self.commands.remove_resource::<CallDos>();
                
            }
        }
    }


    pub fn send_message(&mut self, message: FromClient) {
        dbg!(message.clone());

        let message = bincode::serialize(&message).expect("Failed to serialize message!");
        self.renet_client.send_message(GAME_CHANNEL_ID, message);
    }
}