
use super::MultiplayerState;
use super::InterfaceManager;

mod dealing;

pub use dealing::{delayed_dealing_system,deal_out_cards};


/* .add_system(
    dealing::delayed_dealing_system
    .run_in_state(GameState::InGame)
) */