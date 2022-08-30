use dos_shared::dos_game::DosGame;
use dos_shared::messages::game::{FromClient, GameAction};
use dos_shared::table::Location;
use dos_shared::transfer::{CardTransfer, Table};

use crate::game::GameState;
use crate::game::graphics::{DeckBuilder, CARD_BACK_SPRITE_INDEX};
use crate::game::graphics::constants::DECK_LOCATION;
use crate::game::networking::GameNetworkManager;
use crate::game::client_game::ClientGame;

use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;
use iyes_loopless::prelude::*;

pub struct DrawButtonPlugin;

impl Plugin for DrawButtonPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_enter_system(
            GameState::InGame, 
            draw_button_setup
        )
        .add_system(
            draw_button_display_system
            .run_in_state(GameState::InGame)
        )
        .add_system(
            draw_button_clicked_system
            .run_in_state(GameState::InGame)
        );
    }
}

#[derive(Component)]
struct DrawButton;

fn draw_button_setup(
    mut deck_builder: DeckBuilder,
    mut commands: Commands,
) {
    let transform = Transform::from_translation(Vec3{x:DECK_LOCATION.0, y:DECK_LOCATION.1, z:0.1});

    let index = CARD_BACK_SPRITE_INDEX + 1;

    let e = deck_builder.make_pickable_sprite(transform, index);
    commands.entity(e)
    .insert(Visibility{is_visible: false})
    .insert(DrawButton);
}

fn draw_button_display_system(
    game: ClientGame,
    mut query: Query<&mut Visibility, With<DrawButton>>,
) {
    if game.get_table(&Location::Deck).is_empty() {
        // Toggle On
        for mut visibility in query.iter_mut() {
            visibility.is_visible = true;
        }
    } else {
        // Toggled Off
        for mut visibility in query.iter_mut() {
            visibility.is_visible = false;
        }
    }
}

fn draw_button_clicked_system (
    mut events: EventReader<PickingEvent>,
    buttons: Query<Entity, With<DrawButton>>,
    mut network_manager: GameNetworkManager,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            for button_entity in &buttons {
                if *e == button_entity && 
                !network_manager.game.has_delayed_transfers() &&
                network_manager.game.validate_draw_cards(network_manager.game.mp_state.turn_id) {

                    // Send a message to execute action on server
                    network_manager.send_message(FromClient(GameAction::DrawCards));
                }
            }
        }
    }
}