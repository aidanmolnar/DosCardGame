mod call_dos;
mod client_game;
mod graphics;
mod input;
mod networking;
mod setup_table;
mod sync;
mod table;

pub use call_dos::CallDos;
pub use graphics::AssetState;

use dos_shared::{messages::lobby::GameSnapshot, GameInfo};

use super::{GameState, MultiplayerState};

use bevy::prelude::*;
use bevy_mod_picking::{InteractablePickingPlugin, PickingEvent, PickingPlugin};
use iyes_loopless::prelude::*;

// Resources and systems for in-game
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin)
            // Spawn tables for managing state
            .add_plugin(setup_table::ClientTableSetupPlugin)
            // Create resource for controlling turn advancement
            .add_exit_system(GameState::MainMenu, insert_game_info)
            // Create resource for caching cards that become visible
            .init_resource::<sync::ClientSyncer>()
            // Plugin for assets, camera, and animations
            .add_plugin(graphics::GraphicsPlugin)
            // Handle messages from server
            .add_system(networking::game_network_system.run_in_state(GameState::InGame))
            // Handle input from clients
            .add_plugin(input::WildCardPlugin)
            .add_plugin(input::DrawButtonPlugin)
            .add_plugin(input::CallDosPlugin)
            .add_system(
                input::play_card_system
                    .run_in_state(GameState::InGame)
                    .run_on_event::<PickingEvent>(),
            );
    }
}

fn insert_game_info(
    mut commands: Commands,
    mp_state: Res<MultiplayerState>,
    snapshot_opt: Option<Res<GameSnapshot>>,
) {
    if let Some(snapshot) = snapshot_opt {
        // Accept from server (when reconnecting)
        commands.insert_resource(snapshot.game_info.clone());
    } else {
        // Create initial state
        commands.insert_resource(GameInfo::new(mp_state.player_names.len()));
    }
}
