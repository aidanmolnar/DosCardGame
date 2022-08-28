use crate::game::assets::CardHandles;
use crate::game::card_indexing::CARD_BACK_SPRITE_INDEX;
use crate::game::layout::constants::DECK_LOCATION;
use crate::game::animations::components::{BoardPosition, MouseOffset, LinearAnimation};
use crate::game::table::client_table::ClientItem;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::PickableBundle;


#[derive(SystemParam)]
pub struct DeckBuilder<'w, 's> {
    commands: Commands<'w,'s>,
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<ColorMaterial>>,
    texture_atlases: Res<'w, Assets<TextureAtlas>>,
    card_handles: Res<'w, CardHandles>,
}

impl<'w, 's> DeckBuilder<'w, 's> {
    pub fn make_cards(&mut self, num_cards: usize) -> Vec<ClientItem> {

        let mut entities = Vec::new();

        for i in 0..num_cards {

            let translation = Vec3::new(DECK_LOCATION.0,DECK_LOCATION.1, 0.1 * i as f32);
            let transform = Transform::from_translation(translation).with_scale(Vec3::splat(1.0));

            let e = self.make_pickable_sprite(transform, CARD_BACK_SPRITE_INDEX);
            
            self.commands.entity(e)
            .insert(
                LinearAnimation {
                    start: transform,
                    end: transform,
                    timer: Timer::from_seconds(0.01, false),
                }
            )
            .insert(
                BoardPosition {
                    position: translation
                }
            ).insert(
                MouseOffset {
                    offset: Vec3::ZERO,
                    scale: 1.,
                }
            );

            entities.push(ClientItem(None, e))
        }

        entities
    }


    pub fn make_pickable_sprite(&mut self, transform: Transform, index: usize) -> Entity {
        self.commands.spawn()
        .insert_bundle(
            SpriteSheetBundle {
                sprite: TextureAtlasSprite { 
                    index, 
                    ..default() 
                },
                texture_atlas: self.texture_atlases.get_handle(&self.card_handles.atlas),
                transform,
                ..default()
        }).insert_bundle(
            MaterialMesh2dBundle {
                mesh: self.meshes.add(Mesh::from(shape::Quad::new(Vec2::new(240.,360.)))).into(),
                material: self.materials.add(ColorMaterial::from(Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0. })),
                transform,
                ..default()
            })
        .insert_bundle(PickableBundle::default())
        .id()
    }
}