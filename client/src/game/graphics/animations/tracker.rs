use bevy_sprite3d::{AtlasSprite3d, Sprite3dParams};
use dos_shared::{
    cards::Card,
    table::{CardReference, HandPosition, Location},
    table_map::TableMap,
    transfer::CardTransfer,
    GameState,
};

use crate::{
    game::{call_dos::CallDos, graphics::assets::CardHandles},
    postgame::Victory,
};

use super::{
    card_indexing::{SpriteIndex, CARD_BACK_SPRITE_INDEX},
    core::components::MouseOffset,
    table::{AnimationItem, AnimationTable},
};

use bevy::{ecs::system::SystemParam, prelude::*};
use iyes_loopless::state::NextState;

use std::collections::VecDeque;

#[derive(SystemParam)]
pub struct AnimationTracker<'w, 's> {
    map: Res<'w, TableMap>,
    tables: Query<'w, 's, &'static mut AnimationTable>,
    pub animation_queue: ResMut<'w, AnimationActionQueue>,
    transforms: Query<'w, 's, &'static Transform>,

    commands: Commands<'w, 's>,
    sprite_params: Sprite3dParams<'w, 's>,
    card_handles: Res<'w, CardHandles>,
}

// Stores game events in a queue, so they can be animated sequentially instead of all at once
#[derive(Default, Resource)]
pub struct AnimationActionQueue {
    queue: VecDeque<DelayedAnimationAction>,
    timer: Timer,
}

pub struct DelayedAnimationAction {
    pub action: AnimationAction,
    pub delay: f32, // How long to wait before executing the next action
}

pub enum AnimationAction {
    Transfer {
        from: CardReference,
        to: CardReference,
        card: Option<Card>,
    },
    SetDiscardLast {
        card: Option<Card>,
    },
    Victory {
        winner: usize,
    },
    SomeoneHasTwoCards,
}

// Wait between actions and execute them after delay
pub fn update_animation_actions(mut animation_tracker: AnimationTracker, time: Res<Time>) {
    animation_tracker.animation_queue.timer.tick(time.delta());

    if animation_tracker.animation_queue.timer.finished() {
        if let Some(delayed_action) = animation_tracker.animation_queue.queue.pop_front() {
            match delayed_action.action {
                AnimationAction::Transfer { from, to, card } => {
                    animation_tracker.transfer(&from, &to, card);
                }
                AnimationAction::SetDiscardLast { card } => {
                    animation_tracker.set_discard_last(card);
                }
                AnimationAction::Victory { winner } => {
                    animation_tracker.victory(winner);
                }
                AnimationAction::SomeoneHasTwoCards => {
                    animation_tracker.someone_has_two_cards();
                }
            }

            animation_tracker.animation_queue.timer =
                Timer::from_seconds(delayed_action.delay, TimerMode::Once);
        }
    }
}

impl AnimationTracker<'_, '_> {
    // Change the sprite of a card
    pub fn set_sprite(
        &mut self,
        item: &AnimationItem,
        new_card: Option<Card>, // None indicates facedown card
    ) {
        //let mut sprite = self.sprites.get_mut(item.1).unwrap();
        let index = new_card.map_or(CARD_BACK_SPRITE_INDEX, |card| card.get_sprite_index());

        let transform = self
            .transforms
            .get(item.1)
            .expect("Entity did not have transform");

        self.commands.entity(item.1).insert(
            AtlasSprite3d {
                atlas: self.card_handles.atlas.clone(),

                pixels_per_metre: 1.,
                partial_alpha: true,
                unlit: true,

                transform: *transform,
                index,

                // transform: Transform::from_xyz(0., 0., 0.),
                // pivot: Some(Vec2::new(0.5, 0.5)),
                ..default()
            }
            .bundle(&mut self.sprite_params),
        );
    }

    pub fn reset_mouse_offset(&mut self, item: &AnimationItem) {
        self.commands.entity(item.1).insert(MouseOffset {
            offset: Vec3::ZERO,
            scale: 1.,
        });
    }
}

impl CardTransfer<AnimationItem, AnimationTable> for AnimationTracker<'_, '_> {
    fn get_table(&self, location: &Location) -> &AnimationTable {
        let entity = *self
            .map
            .0
            .get(location)
            .expect("Table entity not found for location");
        self.tables
            .get(entity)
            .expect("ClientTable does not exist for table entity")
    }

    fn get_table_mut(&mut self, location: &Location) -> &mut AnimationTable {
        let entity = *self
            .map
            .0
            .get(location)
            .expect("Table entity not found for location");
        self.tables
            .get_mut(entity)
            .expect("ClientTable does not exist for table entity")
            .into_inner()
    }
}

impl AnimationTracker<'_, '_> {
    pub fn enque_action(&mut self, action: DelayedAnimationAction) {
        self.animation_queue.queue.push_back(action);
    }

    fn transfer(&mut self, from: &CardReference, to: &CardReference, card: Option<Card>) {
        let mut item = self.remove(from).expect("Item did not exist");

        self.reset_mouse_offset(&item);
        self.set_sprite(&item, card);
        item.0 = card;

        self.push(to, item);
    }

    // Changes the value of the discard pile.  Used for setting wild card colors.
    fn set_discard_last(&mut self, card: Option<Card>) {
        let discard = self
            .get_mut(&CardReference {
                location: Location::DiscardPile,
                hand_position: HandPosition::Last,
            })
            .expect("No discarded card");

        discard.0 = card;
        let item = *discard;

        self.set_sprite(&item, card);
    }

    pub fn is_empty(&self) -> bool {
        self.animation_queue.queue.is_empty()
    }

    fn victory(&mut self, winner: usize) {
        println!("player with id {winner} won the game!");
        self.commands.insert_resource(Victory { winner });
        self.commands
            .insert_resource(NextState(GameState::PostGame));
    }

    pub fn someone_has_two_cards(&mut self) {
        self.commands.init_resource::<CallDos>();
    }
}
