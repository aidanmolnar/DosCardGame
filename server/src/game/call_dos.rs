use dos_shared::{
    messages::game::{GameAction, CallDosInfo}, 
    dos_game::DosGame
};

use super::networking::GameNetworkManager;

use bevy::prelude::*;

// Resource for handling "call dos" action when player has two cards remaining
// If player with two cards calls it first, there is no penalty
// If other player calls it first, then the player with two cards must draw cards as a penalty
// Player with two cards gets extra time once someone else has called it to account for network delay
pub struct CallDos {
    pub player: usize, // Player with two cards remaining
    pub caller: Option<usize>, // Player who called out above
    pub graceperiod: Option<Timer>, // Starts once someone calls out players.  Stops when graceperiod is over
}

// Handles ticking down the graceperiod.
//   TODO: clean up as_refs and unwraps.  Currently needed because network_manager owns an option of the CallDos resource.
pub fn call_dos_graceperiod_system(
    mut network_manager: GameNetworkManager,
    time: Res<Time>
) {
    if let Some(graceperiod) = &mut network_manager.game.call_dos.as_mut().unwrap().graceperiod {

        if graceperiod.finished() {
            // Player with two cards did not respond before grace period expired and must draw cards
            network_manager.game.punish_missed_dos(network_manager.game.call_dos.as_ref().unwrap().player);
            network_manager.send_to_all(GameAction::CallDos(Some(CallDosInfo {
                player: network_manager.game.call_dos.as_ref().unwrap().player,
                caller: network_manager.game.call_dos.as_ref().unwrap().caller.expect("Caller must be set when timer is set"),
            })));
            network_manager.game.commands.remove_resource::<CallDos>();
        } else {
            // Grace period still open
            graceperiod.tick(time.delta());
        }
    }
}