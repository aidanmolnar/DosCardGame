use super::networking::GameNetworkManager;

use bevy::prelude::*;
use dos_shared::{messages::game::{GameAction, CallDosInfo}, dos_game::DosGame};

pub struct CallDos {
    pub player: usize,
    pub caller: Option<usize>,
    pub graceperiod: Option<Timer>,
}

// Run if CallDos exists
// TODO: clean up as_refs and unwraps.  Currently needed because network_manager owns an option of the CallDos resource.
pub fn call_dos_graceperiod(
    mut network_manager: GameNetworkManager,
    time: Res<Time>
) {
    if let Some(graceperiod) = &mut network_manager.call_dos.as_mut().unwrap().graceperiod {
        if graceperiod.finished() {

            network_manager.game.punish_missed_dos(network_manager.call_dos.as_ref().unwrap().player);
            network_manager.send_to_all(GameAction::CallDos(Some(CallDosInfo {
                player: network_manager.call_dos.as_ref().unwrap().player,
                caller: network_manager.call_dos.as_ref().unwrap().caller.expect("Caller must be set when timer is set"),
            })));
            network_manager.game.commands.remove_resource::<CallDos>();
        } else {
            graceperiod.tick(time.delta());
        }
    }
}