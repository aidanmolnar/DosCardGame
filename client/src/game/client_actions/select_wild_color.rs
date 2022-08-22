use dos_shared::cards::{Card,CardColor, CardType};
use dos_shared::messages::game::FromClient;

use crate::game::GameState;

use crate::game::card_indexing::SpriteIndex;
use crate::game::networking::GameNetworkManager;
use crate::game::{table::DeckBuilder, animations::components::LinearAnimation};

use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;
use iyes_loopless::prelude::*;

pub struct WildCardPlugin;

impl Plugin for WildCardPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(WildCardManager(false))

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

pub struct WildCardManager( bool);

impl WildCardManager {
    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }
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
            target_position: Vec3{x: 0., y: -300., z: 0.} 
        }
    );
    make_wildcard_button(
        &mut deck_builder, 
        &mut commands, 
        WildCardButton { 
            color: CardColor::Blue, 
            target_position: Vec3{x: -300., y: 0., z: 0.} 
        }
    );
    make_wildcard_button(
        &mut deck_builder, 
        &mut commands, 
        WildCardButton { 
            color: CardColor::Green, 
            target_position: Vec3{x: 0., y: 300., z: 0.} 
        }
    );
    make_wildcard_button(
        &mut deck_builder, 
        &mut commands, 
        WildCardButton { 
            color: CardColor::Yellow, 
            target_position: Vec3{x: 300., y: 0., z: 0.} 
        }
    );
}

fn make_wildcard_button(
    deck_builder: &mut DeckBuilder,
    commands: &mut Commands,
    wildcard_button: WildCardButton
) {
    let transform = Transform::from_translation(Vec3::ZERO);

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
    button_manager: Res<WildCardManager>,
    mut query: Query<(&WildCardButton, &mut Visibility, &mut LinearAnimation)>,
) {
    if button_manager.is_changed() {
        if button_manager.0 {
            // Toggle On
            for (button, mut visibility, mut animation) in query.iter_mut() {
                visibility.is_visible = true;
                animation.end = Transform::from_translation(button.target_position);
            }
        } else {
            // Toggled Off
            for (_, mut visibility, mut animation) in query.iter_mut() {
                visibility.is_visible = false;
                animation.end = Transform::from_translation(Vec3::ZERO);
            }
        }
    }
}

fn wildcard_button_clicked_system (
    mut events: EventReader<PickingEvent>,
    buttons: Query<(Entity, &WildCardButton)>,
    mut button_manager: ResMut<WildCardManager>,
    mut network_manager: GameNetworkManager,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            for (button_entity, button) in &buttons {
                if *e == button_entity {
                    button_manager.toggle();
                    network_manager.game_info.next_turn();

                    // Update the local card color
                    let mut card = network_manager.card_transferer.peek_discard().unwrap();
                    card.color = button.color;

                    network_manager.card_transferer.set_discard_value(Some(card));
                    println!("{:?}", button.color);

                    // Send a message with the card color to the server
                    network_manager.send_message(FromClient::DiscardWildColor(button.color));
                }
            }
        }
    }
}