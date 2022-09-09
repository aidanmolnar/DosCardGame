use dos_shared::GameInfo;
use dos_shared::messages::lobby::GameSnapshot;

use super::GameState;
use super::MultiplayerState;

mod client_game;
mod input;
mod networking;
mod table;
mod graphics;
mod setup_table;
mod sync;
mod call_dos;

pub use call_dos::CallDos;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_mod_picking::{PickingPlugin,InteractablePickingPlugin, PickingEvent };

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)

        // Spawn tables for managing state
        .add_plugin(setup_table::ClientTableSetupPlugin)

        .add_plugin(graphics::GraphicsPlugin)

        // Create resource for controlling turn advancement
        .add_exit_system(
            GameState::MainMenu, 
            insert_game_info
        )

        // Clear optional game resources when exiting to main menu
        .add_enter_system(
            GameState::MainMenu, 
            |mut commands: Commands| {commands.remove_resource::<CallDos>()}
        )

        
        // Create resource for caching cards that become visible
        .init_resource::<sync::ClientSyncer>()

        // Handle messages from server
        .add_system(networking::game_network_system
            .run_in_state(GameState::InGame)
        )
        
        // Handle input from clients
        .add_plugin(input::WildCardPlugin)
        .add_plugin(input::DrawButtonPlugin)
        .add_plugin(input::CallDosPlugin)
        .add_system(input::play_card_system
            .run_in_state(GameState::InGame)
            .run_on_event::<PickingEvent>()
        );
    }

}

fn insert_game_info(
    mut commands: Commands, 
    mp_state: Res<MultiplayerState>,
    snapshot_opt: Option<Res<GameSnapshot>>,
) {
    if let Some(snapshot) = snapshot_opt {
        commands.insert_resource(snapshot.game_info.clone())
    } else {
        commands.insert_resource(
            GameInfo::new(mp_state.player_names.len())
        )
    }
}