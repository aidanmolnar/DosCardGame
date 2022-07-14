// TODO: very cursed import paths
use super::super::layout::constants::*;
use super::super::client_actions::play_card::CardValue;

use super::components::*;
use super::card_indexing::*;
use super::assets::CardHandles;

use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy::sprite::MaterialMesh2dBundle;

// Add a card sprite to tagged entities
pub fn card_blueprint_system(
    mut query: Query<(Entity, Option<&mut TextureAtlasSprite>, &CardBlueprint)>,
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    card_handles: Res<CardHandles>,
) {
    for (entity, option_texture_atlas, blueprint) in query.iter_mut() {
        // Reuse existing tetxure atlas if possible
        // TODO: this might not be necessary...
        if let Some(mut texture_atlas) = option_texture_atlas {
            texture_atlas.index = blueprint.card.get_sprite_index();
        } else {
            commands.entity(entity)
            .insert_bundle(
                SpriteSheetBundle {
                    sprite: TextureAtlasSprite { 
                        index: blueprint.card.get_sprite_index(), 
                        ..default() 
                    },
                    texture_atlas: texture_atlases.get_handle(&card_handles.atlas),
                    transform: Transform::from_translation(Vec3::new(DECK_LOCATION.0,DECK_LOCATION.1,0.)).with_scale(Vec3::splat(1.0)),
                    ..default()
            });
        }

        if let Some(value) = blueprint.card{
            commands.entity(entity).insert(CardValue(value));
        }

        commands.entity(entity).remove::<CardBlueprint>();
    }
}

// Add a pickable mesh to tagged entities
pub fn pickable_blueprint_system(
    query: Query<Entity, With<PickableBlueprint>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in query.iter() {
        commands.entity(entity)
        .insert_bundle(
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(240.,360.)))).into(),
                material: materials.add(ColorMaterial::from(Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0. })),
                transform: Transform::from_translation(Vec3::new(DECK_LOCATION.0,DECK_LOCATION.1,0.)).with_scale(Vec3::splat(1.0)),
                ..default()
            })
        .insert_bundle(PickableBundle::default())
        .remove::<PickableBlueprint>();
    }
}