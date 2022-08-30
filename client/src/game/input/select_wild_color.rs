use dos_shared::cards::{Card,CardColor, CardType};
use dos_shared::dos_game::{DosGame, TurnState};
use dos_shared::messages::game::{FromClient, GameAction};

use crate::game::GameState;
use crate::game::graphics::components::LinearAnimation;
use crate::game::graphics::{DeckBuilder, SpriteIndex};
use crate::game::networking::GameNetworkManager;
use crate::game::client_game::ClientGame;

use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;
use iyes_loopless::prelude::*;

const BUTTON_START: (f32,f32,f32) = (0.,0.,200.);

pub struct WildCardPlugin;

impl Plugin for WildCardPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_enter_system(
            GameState::InGame, 
            wildcard_select_setup
        )
        .add_system(
            wildcard_button_clicked_system
            .run_in_state(GameState::InGame)
        )
        .add_system(
            wildcard_button_display_system
            .run_in_state(GameState::InGame)
        );
    }
}

#[derive(Component)]
struct WildCardButton {
    color: CardColor,
    target_position: Vec3
}

fn wildcard_select_setup(
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

fn make_wildcard_button(
    deck_builder: &mut DeckBuilder,
    commands: &mut Commands,
    wildcard_button: WildCardButton
) {
    let transform = Transform::from_translation(Vec3{x:BUTTON_START.0,y:BUTTON_START.1,z:BUTTON_START.2});

    let index = Card{color: wildcard_button.color, ty: CardType::Wild}.get_sprite_index();

    let e = deck_builder.make_pickable_sprite(transform, index);
    commands.entity(e)
    .insert(Visibility{is_visible: false})
    .insert(LinearAnimation{
        start: transform, 
        end: transform, 
        timer: Timer::from_seconds(0.01, false)
    })
    .insert(wildcard_button);
}

fn wildcard_button_display_system(
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
                animation.timer = Timer::from_seconds(0.4, false);
            }
        } else {
            // Toggled Off
            for (_, mut visibility, mut animation, transform) in query.iter_mut() {
                visibility.is_visible = false;
                animation.start = *transform;
                animation.end = Transform::from_translation(Vec3{x:BUTTON_START.0,y:BUTTON_START.1,z:BUTTON_START.2});
                animation.timer = Timer::from_seconds(0.01, false);
            }
        }

        *previous_turn_state = turn_state;
    }
}

fn wildcard_button_clicked_system (
    mut events: EventReader<PickingEvent>,
    buttons: Query<(Entity, &WildCardButton)>,
    mut network_manager: GameNetworkManager,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            for (button_entity, button) in &buttons {
                if *e == button_entity && 
                !network_manager.game.has_delayed_transfers() &&
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