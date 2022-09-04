use bevy::sprite::MaterialMesh2dBundle;
use dos_shared::messages::game::{FromClient, GameAction};

use crate::game::GameState;
use crate::game::graphics::DosButtonHandle;
use crate::game::graphics::constants::DECK_LOCATION;
use crate::game::networking::GameNetworkManager;
use crate::game::call_dos::CallDos;

use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, PickableBundle};
use iyes_loopless::prelude::*;

pub struct CallDosPlugin;

impl Plugin for CallDosPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_enter_system(
            GameState::InGame, 
            call_dos_button_setup
        )
        .add_exit_system(
            GameState::InGame, 
            call_dos_button_cleanup
        )
        .add_system(
            call_dos_button_display_system
            .run_in_state(GameState::InGame)
        )
        .add_system(
            call_dos_button_clicked_system
            .run_in_state(GameState::InGame)
        );
    }
}

#[derive(Component)]
struct CallDosButton;

fn call_dos_button_setup(
    mut commands: Commands,
    handles: Res<DosButtonHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let transform = Transform::from_translation(Vec3{x: DECK_LOCATION.0+ 300., y:DECK_LOCATION.1, z:0.1});

    commands.spawn()
    .insert_bundle(
        SpriteBundle {
            texture: handles.texture.clone(),
            transform,
            ..default()
    }).insert_bundle(
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(360.,360.)))).into(),
            material: materials.add(ColorMaterial::from(Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0. })),
            transform,
            ..default()
        })
    .insert_bundle(PickableBundle::default())
    .insert(Visibility{is_visible: false})
    .insert(CallDosButton);
}

fn call_dos_button_display_system(
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

fn call_dos_button_clicked_system (
    mut events: EventReader<PickingEvent>,
    buttons: Query<Entity, With<CallDosButton>>,
    mut network_manager: GameNetworkManager,
    call_dos_res: Option<Res<CallDos>>,
    mut commands: Commands,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            for button_entity in &buttons {
                if *e == button_entity && 
                !network_manager.game.has_delayed_transfers() &&
                call_dos_res.is_some() // TODO: might not need to check this
                { 
                    // Send a message to execute action on server
                    network_manager.send_message(FromClient(GameAction::CallDos(None)));
                    commands.remove_resource::<CallDos>();
                }
            }
        }
    }
}

fn call_dos_button_cleanup(
    mut commands: Commands,
    cards: Query<Entity, With<CallDosButton>>,
) {
    for entity in &cards {
        commands.entity(entity).despawn();
    }
}