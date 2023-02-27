use dos_shared::cards::Card;

use super::animations::components::{BoardPosition, LinearAnimation, MouseOffset};
use super::animations::AnimationItem;
use super::assets::CardHandles;
use super::card_indexing::CARD_BACK_SPRITE_INDEX;
use super::layout::constants::DECK_LOCATION;
use super::SpriteIndex;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy_sprite3d::{AtlasSprite3d, Sprite3d, Sprite3dParams};

// Builds card entities from assets and resource handles
#[derive(SystemParam)]
pub struct DeckBuilder<'w, 's> {
    commands: Commands<'w, 's>,
    card_handles: Res<'w, CardHandles>,
    sprite_params: Sprite3dParams<'w, 's>,
}

impl<'w, 's> DeckBuilder<'w, 's> {
    // Create a new vector of face-down cards at the deck location
    pub fn make_unknown_cards(&mut self, num_cards: usize) -> Vec<AnimationItem> {
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
    pub fn make_known_cards(&mut self, cards: Vec<Card>) -> Vec<AnimationItem> {
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
    fn make_card(&mut self, sprite_index: usize, z: f32) -> Entity {
        let translation = Vec3::new(DECK_LOCATION.0, DECK_LOCATION.1, z);
        let transform = Transform::from_translation(translation).with_scale(Vec3::splat(1.0));

        let entity = self.make_pickable_card_sprite(transform, sprite_index);
        let glow = self.make_glow();

        // Add animation components
        self.commands
            .entity(entity)
            .insert(LinearAnimation {
                start: transform,
                end: transform,
                timer: Timer::from_seconds(0.01, TimerMode::Once),
            })
            .insert(BoardPosition {
                position: translation,
            })
            .insert(MouseOffset {
                offset: Vec3::ZERO,
                scale: 1.,
            })
            .add_child(glow);

        entity
    }

    pub fn make_glow(&mut self) -> Entity {
        let bundle = Sprite3d {
            image: self.card_handles.glow.clone(),
            pixels_per_metre: 1.,
            partial_alpha: true,
            unlit: true,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -0.005)),
            ..default()
        }
        .bundle(&mut self.sprite_params);

        // Update material color
        // TODO: Currently this unecessarily updates the color for each glow created (only needs to happen once)
        let handle = bundle.pbr.material.clone();
        let mut material = self.sprite_params.materials.get_mut(&handle).unwrap();
        material.base_color = Color::BLUE;

        self.commands
            .spawn(bundle)
            .insert(VisibilityBundle {
                visibility: Visibility { is_visible: false },
                ..default()
            })
            .id()
    }

    // Spawns a card without animation components (used to make buttons)
    pub fn make_pickable_card_sprite(&mut self, transform: Transform, index: usize) -> Entity {
        self.commands
            .spawn(
                AtlasSprite3d {
                    atlas: self.card_handles.atlas.clone(),
                    pixels_per_metre: 1.,
                    partial_alpha: true,
                    unlit: true,
                    transform,
                    index,
                    ..default()
                }
                .bundle(&mut self.sprite_params),
            )
            .insert(PickableBundle::default())
            .id()
    }
}
