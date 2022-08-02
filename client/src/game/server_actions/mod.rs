
use super::MultiplayerState;

pub mod dealing;

pub use dealing::{deal_out_cards, delayed_dealing_system};


/* .add_system(
    dealing::delayed_dealing_system
    .run_in_state(GameState::InGame)
) */