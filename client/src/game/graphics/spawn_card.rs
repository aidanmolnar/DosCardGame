use dos_shared::cards::*;

use super::interface_constants::*;
use super::card_indexing::SpriteIndex;
use super::assets::CardHandles;
use super::animations::{Target, LinearAnimation};

use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy::sprite::MaterialMesh2dBundle;

pub struct SpawnCardSystems;

impl Plugin for SpawnCardSystems {
    fn build(&self, app: &mut App) {
        app
        .add_system_to_stage(CoreStage::PostUpdate, build_cards)
        .add_system_to_stage(CoreStage::PostUpdate, build_pickable);
    }
}

#[derive(Component)]
struct CardBlueprint {
    card: Option<Card>,
}

#[derive(Component)]
struct PickableBlueprint;


pub fn spawn_card_entity(
    card: Option<Card>,
    pickable: bool,
    commands: &mut Commands,
) -> Entity {
    let mut entity_commands = commands.spawn();

    entity_commands.insert(CardBlueprint{card});

    if pickable {
        entity_commands.insert(PickableBlueprint);
    }

    entity_commands.id()
}

fn build_cards (
    query: Query<(Entity, &CardBlueprint)>,
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    card_handles: Res<CardHandles>,
) {
    let translation = Vec3::new(DECK_LOCATION.0, DECK_LOCATION.1, 0.);

    for (entity, blueprint) in query.iter() {
        commands.entity(entity).insert_bundle(
            SpriteSheetBundle {
                sprite: TextureAtlasSprite { 
                    index: blueprint.card.get_sprite_index(), 
                    ..default() 
                },
                texture_atlas: texture_atlases.get_handle(&card_handles.atlas),
                transform: Transform::from_translation(translation).with_scale(Vec3::splat(1.0)),
                ..default()
        }).insert( LinearAnimation {
            start: Transform::from_translation(translation),
            end: Transform::from_translation(translation),
            timer: Timer::from_seconds(0.01,false),
        }).insert( Target {
            target: translation,
        }).remove::<CardBlueprint>();
    }
}

// TODO: Only build this mesh/material once and keep a handle to it...
fn build_pickable (
    query: Query<Entity, With<PickableBlueprint>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in query.iter() {
        commands.entity(entity)
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(240.,360.)))).into(),
                material: materials.add(ColorMaterial::from(Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0. })),
                ..default()
            }).insert_bundle(PickableBundle::default())
            .remove::<PickableBlueprint>();
    }
}
