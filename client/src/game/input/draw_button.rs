use dos_shared::{
    dos_game::DosGame, 
    messages::game::{FromClient, GameAction}, 
    table::{Location, Table}, 
    transfer::CardTransfer
};

use crate::game::{
    GameState, 
    graphics::{DeckBuilder, CARD_BACK_SPRITE_INDEX, constants::DECK_LOCATION},
    networking::GameNetworkManager, 
    client_game::ClientGame
};

use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;
use iyes_loopless::prelude::*;

// Adds a button for drawing cards when the deck is empty.
// Typically this is handled by clicking on a card in the deck, but this is important for scenarios where the deck is out of cards.  
pub struct DrawButtonPlugin;

impl Plugin for DrawButtonPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_enter_system(
            GameState::InGame, 
            setup_system
        )
        .add_exit_system(
            GameState::InGame, 
            cleanup_system
        )
        .add_system(
            display_system
            .run_in_state(GameState::InGame)
        )
        .add_system(
            clicked_system
            .run_in_state(GameState::InGame)
        );
    }
}

#[derive(Component)]
struct DrawButton;

fn setup_system(
    mut deck_builder: DeckBuilder,
    mut commands: Commands,
) {
    let transform = Transform::from_translation(Vec3{x:DECK_LOCATION.0, y:DECK_LOCATION.1, z:0.1});

    let index = CARD_BACK_SPRITE_INDEX + 1;

    let e = deck_builder.make_pickable_card_sprite(transform, index);
    commands.entity(e)
    .insert(Visibility{is_visible: false})
    .insert(DrawButton);
}

// Turns the button on and off
fn display_system(
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

// Handling when the button is clicked
fn clicked_system (
    mut events: EventReader<PickingEvent>,
    buttons: Query<Entity, With<DrawButton>>,
    mut network_manager: GameNetworkManager,
) {
    // Iterates over all click events
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {

            // Checks if the click was on the button
            for button_entity in &buttons {
                if *e == button_entity && 
                !network_manager.game.has_delayed_transfers() &&
                // Checks if the action is allowed
                network_manager.game.validate_draw_cards(network_manager.game.mp_state.turn_id) {

                    // Send a message to execute action on server
                    network_manager.send_message(FromClient(GameAction::DrawCards));
                }
            }
        }
    }
}

fn cleanup_system(
    mut commands: Commands,
    cards: Query<Entity, With<DrawButton>>,
) {
    for entity in &cards {
        commands.entity(entity).despawn();
    }
}