use super::SpriteIndex;
use super::animations::AnimationItem;
use super::assets::CardHandles;
use super::card_indexing::CARD_BACK_SPRITE_INDEX;
use super::layout::constants::DECK_LOCATION;
use super::animations::components::{BoardPosition, MouseOffset, LinearAnimation};

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::PickableBundle;
use dos_shared::cards::Card;

// Builds card entities from assets and resource handles
#[derive(SystemParam)]
pub struct DeckBuilder<'w, 's> {
    commands: Commands<'w,'s>,
    meshes: Res<'w, Assets<Mesh>>,
    materials: Res<'w, Assets<ColorMaterial>>,
    texture_atlases: Res<'w, Assets<TextureAtlas>>,
    card_handles: Res<'w, CardHandles>,
}

impl<'w, 's> DeckBuilder<'w, 's> {

    // Create a new vector of face-down cards at the deck location
    pub fn make_unknown_cards(
        &mut self, 
        num_cards: usize
    ) -> Vec<AnimationItem> {

        let mut items = Vec::new();

        for i in 0..num_cards {
            #[allow(clippy::cast_precision_loss)]
            let z = 0.1 * i as f32;

            let entity = self.make_card(CARD_BACK_SPRITE_INDEX, z);

            items.push(AnimationItem(None, entity));
        }

        items
    }

    // Create a new vector of face-up cards at the deck location
    pub fn make_known_cards(
        &mut self, 
        cards: Vec<Card>
    ) -> Vec<AnimationItem> {

        let mut items = Vec::new();

        for (i, card) in cards.iter().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let z = 0.1 * i as f32;

            let entity = self.make_card(card.get_sprite_index(), z);

            items.push(AnimationItem(Some(*card), entity));
        }

        items
    }

    // Spawns a single card
    fn make_card(
        &mut self, 
        sprite_index: usize, 
        z: f32
    ) -> Entity {

        let translation = Vec3::new(DECK_LOCATION.0,DECK_LOCATION.1, z);
        let transform = Transform::from_translation(translation).with_scale(Vec3::splat(1.0));

        let entity = self.make_pickable_card_sprite(transform, sprite_index);
        
        // Add animation components
        self.commands.entity(entity)
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

        entity
    }

    // Spawns a card without animation components (used to make buttons)
    pub fn make_pickable_card_sprite (
        &mut self, 
        transform: Transform, 
        index: usize
    ) -> Entity {
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
                mesh: self.meshes.get_handle(self.card_handles.mesh.clone()).into(),
                material: self.materials.get_handle(self.card_handles.material.clone()),
                transform,
                ..default()
            })
        .insert_bundle(PickableBundle::default())
        .id()
    }
}