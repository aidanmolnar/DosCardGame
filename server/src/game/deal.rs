use dos_shared::{
    dos_game::DosGame, 
    DECK_SIZE, 
    messages::game::GameAction
};

use super::networking::GameNetworkManager;

// Runs when leaving main menu
pub fn deal_out_starting_hands(
    mut network_manager: GameNetworkManager,
) {
    network_manager.game.deal_starting_cards(DECK_SIZE);
    network_manager.send_to_all(GameAction::DealIn);
}