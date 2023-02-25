use bevy_sprite3d::{Sprite3d, Sprite3dParams};
use dos_shared::messages::game::{FromClient, GameAction};

use crate::game::{
    call_dos::CallDos,
    graphics::{constants::DECK_LOCATION, DosButtonHandle},
    networking::GameNetworkManager,
    GameState,
};

use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingEvent};
use iyes_loopless::prelude::*;

// Adds a button for calling when you have two cards left, or that someone else has two cards left and did not call.
// Button only appears when appropriate.
pub struct CallDosPlugin;

impl Plugin for CallDosPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, setup_system)
            .add_exit_system(GameState::InGame, cleanup_system)
            .add_system(display_system.run_in_state(GameState::InGame))
            .add_system(clicked_system.run_in_state(GameState::InGame))
            // Remove game resource when exiting to main menu as well
            .add_enter_system(GameState::MainMenu, |mut commands: Commands| {
                commands.remove_resource::<CallDos>();
            });
    }
}

#[derive(Component)]
struct CallDosButton;

fn setup_system(
    mut commands: Commands,
    handles: Res<DosButtonHandle>,
    mut sprite_params: Sprite3dParams,
) {
    let transform = Transform::from_translation(Vec3 {
        x: DECK_LOCATION.0 + 300.,
        y: DECK_LOCATION.1,
        z: 0.1,
    });

    commands
        .spawn(
            Sprite3d {
                image: handles.texture.clone(),

                pixels_per_metre: 1.,
                partial_alpha: true,
                unlit: true,

                transform,

                ..default()
            }
            .bundle(&mut sprite_params),
        )
        .insert(PickableBundle::default())
        .insert(Visibility { is_visible: false })
        .insert(CallDosButton);
}

// Turns the button on and off
fn display_system(
    call_dos_res: Option<Res<CallDos>>,
    mut query: Query<&mut Visibility, With<CallDosButton>>,
) {
    if call_dos_res.is_some() {
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
fn clicked_system(
    buttons: Query<Entity, With<CallDosButton>>,
    mut network_manager: GameNetworkManager,
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
) {
    // Iterates over all click events
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            // Checks if the click was on the button
            for button_entity in &buttons {
                if *e == button_entity && !network_manager.game.has_delayed_transfers() {
                    // Send a message to execute action on server
                    network_manager.send_message(FromClient(GameAction::CallDos(None)));
                    commands.remove_resource::<CallDos>();
                }
            }
        }
    }
}

fn cleanup_system(mut commands: Commands, cards: Query<Entity, With<CallDosButton>>) {
    for entity in &cards {
        commands.entity(entity).despawn();
    }
}
