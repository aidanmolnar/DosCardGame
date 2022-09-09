use bevy_renet::renet::RenetServer;
use dos_shared::net_config::GAME_CHANNEL_ID;
use dos_shared::dos_game::DosGame;
use dos_shared::messages::game::{CallDosInfo, FromClient, FromServer, GameAction};

use crate::multiplayer::MultiplayerState;

use super::server_game::ServerGame;
use super::call_dos::CallDos;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use::bincode;


#[derive(SystemParam)]
pub struct GameNetworkManager<'w,'s> {
    pub game: ServerGame<'w,'s>,
    pub renet_server: ResMut<'w, RenetServer>,
    mp_state: Res<'w, MultiplayerState>,
}

pub fn game_network_system (
    mut manager: GameNetworkManager,
) {
    for client_id in manager.renet_server.clients_id() {
        while let Some(message) = manager.renet_server.receive_message(client_id, GAME_CHANNEL_ID) {

            let player = manager.mp_state.player_from_renet_id(client_id);

            // TODO: don't expect
            let update= bincode::deserialize(&message)
            .expect("Couldn't deserialize message"); 

            manager.handle_update(
                update,   
                player, 
            );
        }
    }
}

impl<'w,'s> GameNetworkManager<'w,'s> {

    // TODO: Replace panics
    fn handle_update(
        &mut self,
        update: FromClient, 
        player: usize,
    ) {
        dbg!(update.clone());

        match update.0 {
            GameAction::PlayCard (card)=> {
                if self.game.validate_play_card(player, &card) {
                    self.game.play_card(&card);

                    self.send_to_filtered(GameAction::PlayCard(card), |p|p!=player);
                } else {
                    println!("Invalid play card");
                    self.handle_disconnect(player);
                }
            },
            GameAction::DrawCards => {
                if self.game.validate_draw_cards(player) {
                    self.game.draw_cards();

                    self.send_to_all(GameAction::DrawCards);
                } else {
                    println!("Invalid draw cards");
                    self.handle_disconnect(player);
                }
            },
            GameAction::KeepStaging => {
                if self.game.validate_keep_last_drawn_card(player) {
                    self.game.keep_last_drawn_card();

                    self.send_to_filtered(GameAction::KeepStaging, |p|p!=player);
                } else {
                    println!("Invalid keep last drawn card");
                    self.handle_disconnect(player);
                }
            },
            GameAction::DiscardWildColor(color) => {
                if self.game.validate_declare_wildcard_color(player, &color) {
                    self.game.declare_wildcard_color(&color);

                    self.send_to_filtered(GameAction::DiscardWildColor(color), |p|p!=player);
                } else {
                    println!("Invalid wildcard select color");
                    self.handle_disconnect(player);
                }
            },
            GameAction::CallDos{..} => {
                // TODO: Try to clean up
                if let Some(call_dos) = &mut self.game.call_dos {
                    if player == call_dos.player {
                        let action = GameAction::CallDos (
                            Some(CallDosInfo {
                                player: call_dos.player,
                                caller: call_dos.player,
                            })
                        );
                        // Remove call dos, send message that someone called it
                        self.game.commands.remove_resource::<CallDos>();
                        self.send_to_all(action);
                    } else {
                        // Start the timer running if it is not already running
                        if call_dos.graceperiod.is_none() {
                            call_dos.caller = Some(player);
                            call_dos.graceperiod = Some(Timer::from_seconds(0.5, false));
                        }
                    }
                } else {
                    // This isn't necessarily a desync, just client not receiving call dos message yet.
                    println!("Invalid call dos");
                }
            }
            GameAction::DealIn => {
                println!("Invalid client action");
                self.handle_disconnect(player);
            }
        }
    }

    
    pub fn send_to_filtered<F> (
        &mut self,
        action: GameAction,
        filter: F // Takes player_id as argument. Sends message if true.
    ) where F: Fn(usize) -> bool{
        let conditions = self.game.syncer.take_conditions();

        // Loop over playes that meet condition
        for (player, renet_id) in self.mp_state.iter_players()
        .filter(
            |(player,_)| filter(*player)
        ) {
            let cards = self.game.syncer.take_player_cards(player);
            let message = bincode::serialize(&FromServer{
                action: action.clone(), 
                conditions: conditions.clone(), 
                cards
            }).expect("Failed to serialize message");

            self.renet_server.send_message(renet_id, GAME_CHANNEL_ID, message);
        }
    }   

    pub fn send_to_all(
        &mut self,
        action: GameAction,
    ) {
        self.send_to_filtered(action, |_|{true});
    }

    fn handle_disconnect(&mut self, player: usize) {
        let renet_id = self.mp_state.renet_id_from_player(player);
        self.renet_server.disconnect(renet_id);

        println!("Disconnecting {player}");
    }
}







