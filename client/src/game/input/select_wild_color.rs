use dos_shared::{
    cards::{Card,CardColor, CardType}, 
    dos_game::{DosGame, TurnState}, 
    messages::game::{FromClient, GameAction}, 
    GameState
};

use crate::game::{
    graphics::{
        components::LinearAnimation, 
        DeckBuilder, 
        SpriteIndex
    }, 
    networking::GameNetworkManager, 
    client_game::ClientGame
};

use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;
use iyes_loopless::prelude::*;

const BUTTON_START: (f32,f32,f32) = (-200.,0.,200.);

// Adds four buttons for specifying what color a wildcard should be when played.
pub struct WildCardPlugin;

impl Plugin for WildCardPlugin {
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
            clicked_system
            .run_in_state(GameState::InGame)
        )
        .add_system(
            display_system
            .run_in_state(GameState::InGame)
        );
    }
}

#[derive(Component)]
struct WildCardButton {
    color: CardColor,
    target_position: Vec3
}

// Makes a button for each color
fn setup_system(
    mut deck_builder: DeckBuilder,
    mut commands: Commands,
) {
    make_wildcard_button(
        &mut deck_builder, 
        &mut commands, 
        WildCardButton { 
            color: CardColor::Red, 
            target_position: Vec3{x: 0., y: -300., z: 200.} 
        }
    );
    make_wildcard_button(
        &mut deck_builder, 
        &mut commands, 
        WildCardButton { 
            color: CardColor::Blue, 
            target_position: Vec3{x: -300., y: 0., z: 200.} 
        }
    );
    make_wildcard_button(
        &mut deck_builder, 
        &mut commands, 
        WildCardButton { 
            color: CardColor::Green, 
            target_position: Vec3{x: 0., y: 300., z: 200.} 
        }
    );
    make_wildcard_button(
        &mut deck_builder, 
        &mut commands, 
        WildCardButton { 
            color: CardColor::Yellow, 
            target_position: Vec3{x: 300., y: 0., z: 200.} 
        }
    );
}

// Makes a single wildcard select button
fn make_wildcard_button(
    deck_builder: &mut DeckBuilder,
    commands: &mut Commands,
    wildcard_button: WildCardButton
) {
    let transform = Transform::from_translation(Vec3{x:BUTTON_START.0,y:BUTTON_START.1,z:BUTTON_START.2});

    let index = Card{color: wildcard_button.color, ty: CardType::Wild}.get_sprite_index();

    let e = deck_builder.make_pickable_card_sprite(transform, index);
    commands.entity(e)
    .insert(Visibility{is_visible: false})
    .insert(LinearAnimation{
        start: transform, 
        end: transform, 
        timer: Timer::from_seconds(0.01, TimerMode::Once)
    })
    .insert(wildcard_button);
}

// Handles turning the buttons on and off
// Buttons move out from discard pile where wildcard was played
fn display_system(
    card_tracker: ClientGame,
    mut previous_turn_state: Local<TurnState>,
    mut query: Query<(&WildCardButton, &mut Visibility, &mut LinearAnimation, &Transform)>,
) {
    let turn_state = card_tracker.get_turn_state();

    if turn_state != *previous_turn_state {
        // Clean up turn check
        if turn_state == TurnState::WildcardColorSelect && card_tracker.is_players_turn(card_tracker.mp_state.turn_id) {
            // Toggle On
            for (button, mut visibility, mut animation, transform) in query.iter_mut() {
                visibility.is_visible = true;
                animation.start = *transform;
                animation.end = Transform::from_translation(button.target_position);
                animation.timer = Timer::from_seconds(0.4, TimerMode::Once);
            }
        } else {
            // Toggled Off
            for (_, mut visibility, mut animation, transform) in query.iter_mut() {
                visibility.is_visible = false;
                animation.start = *transform;
                animation.end = Transform::from_translation(Vec3{x:BUTTON_START.0,y:BUTTON_START.1,z:BUTTON_START.2});
                animation.timer = Timer::from_seconds(0.01, TimerMode::Once);
            }
        }

        *previous_turn_state = turn_state;
    }
}

// Handling when the button is clicked
fn clicked_system (
    mut events: EventReader<PickingEvent>,
    buttons: Query<(Entity, &WildCardButton)>,
    mut network_manager: GameNetworkManager,
) {
    // Iterates over all click events
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {

            // Checks if the click was on the button
            for (button_entity, button) in &buttons {
                if *e == button_entity && 
                !network_manager.game.has_delayed_transfers() &&
                // Checks if the action is allowed
                network_manager.game.validate_declare_wildcard_color(network_manager.game.mp_state.turn_id, &button.color) {

                    // Update the local card color
                    network_manager.game.declare_wildcard_color(&button.color);

                    // Send a message with the card color to the server
                    network_manager.send_message(FromClient(GameAction::DiscardWildColor(button.color)));
                }
            }
        }
    }
}

fn cleanup_system(
    mut commands: Commands,
    cards: Query<Entity, With<WildCardButton>>,
) {
    for entity in &cards {
        commands.entity(entity).despawn();
    }
}