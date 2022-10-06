use dos_shared::{
    net_config::GAME_CHANNEL_ID, 
    dos_game::DosGame, 
    messages::game::{FromClient, FromServer, GameAction}, 
    DECK_SIZE
};

use super::{client_game::ClientGame, call_dos::CallDos};


// Marker resource spawned when drawing cards before server sends message
pub struct WaitingForCards;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_renet::renet::RenetClient;

#[derive(SystemParam)]
pub struct GameNetworkManager<'w, 's> {
    pub commands: Commands<'w,'s>,
    renet_client: ResMut<'w, RenetClient>,
    pub game: ClientGame<'w,'s>,
}

// Recieves and handles messages from the server
pub fn game_network_system(
    mut network_manager: GameNetworkManager,
) {
    // Read all game messages from server
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

        // Execute action
        match action {
            GameAction::DealIn => {
                 self.game.deal_starting_cards(DECK_SIZE);
            },
            GameAction::PlayCard(card) => {
                self.game.play_card(&card);
            },
            GameAction::DrawCards => {
                self.commands.remove_resource::<WaitingForCards>();
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
                }
                self.commands.remove_resource::<CallDos>();
                
            }
        }
    }


    pub fn send_message(&mut self, message: FromClient) {
        let message = bincode::serialize(&message).expect("Failed to serialize message!");
        self.renet_client.send_message(GAME_CHANNEL_ID, message);
    }
}