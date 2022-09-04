use dos_shared::GameState;
use dos_shared::cards::Card;
use dos_shared::table::{Location, CardReference, HandPosition};
use dos_shared::table_map::TableMap;
use dos_shared::transfer::CardTransfer;
use iyes_loopless::state::NextState;

use crate::game::call_dos::CallDos;
use crate::postgame::Victory;

use super::card_indexing::CARD_BACK_SPRITE_INDEX;
use super::table::{AnimationItem, AnimationTable};
use super::core::components::MouseOffset;
use super::card_indexing::SpriteIndex;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use std::collections::VecDeque;


#[derive(SystemParam)]
pub struct AnimationTracker<'w,'s> {
    map: Res<'w, TableMap>,
    tables: Query<'w, 's, &'static mut AnimationTable>,

    pub animation_queue: ResMut<'w, AnimationActionQueue>,
    
    sprites: Query<'w, 's, &'static mut TextureAtlasSprite>,
    commands: Commands<'w, 's>,
}

#[derive(Default)]
pub struct AnimationActionQueue {
    queue: VecDeque<DelayedAnimationAction>,
    timer: Timer,
}

pub struct DelayedAnimationAction {
    pub action: AnimationAction,
    pub delay: f32,
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

pub fn update_animation_actions(
    mut animation_tracker: AnimationTracker,
    time: Res<Time>,
) {
    animation_tracker.animation_queue.timer.tick(time.delta());

    if animation_tracker.animation_queue.timer.finished() {
        if let Some(delayed_action) = animation_tracker.animation_queue.queue.pop_front() {
            match delayed_action.action {
                AnimationAction::Transfer { from, to, card } => {
                    animation_tracker.transfer(&from, &to, card)
                },
                AnimationAction::SetDiscardLast { card } => {
                    animation_tracker.set_discard_last(card)
                },
                AnimationAction::Victory{winner} => {
                    animation_tracker.victory(winner)
                }
                AnimationAction::SomeoneHasTwoCards => {
                    animation_tracker.someone_has_two_cards();
                },
            }

            animation_tracker.animation_queue.timer = Timer::from_seconds(delayed_action.delay, false);
        }
    }
}

impl AnimationTracker<'_,'_> {
    pub fn set_sprite(&mut self, item: &AnimationItem, new_card: Option<Card>) {
        let mut sprite = self.sprites.get_mut(item.1).unwrap();

        if let Some(card) = new_card {
            sprite.index = card.get_sprite_index();
        } else {
            sprite.index = CARD_BACK_SPRITE_INDEX;
        }
    }

    pub fn reset_mouse_offset(&mut self, item: &AnimationItem) {
        self.commands.entity(item.1).insert(MouseOffset{offset: Vec3::ZERO, scale: 1.});
    }
}

impl CardTransfer<AnimationItem, AnimationTable> for AnimationTracker<'_, '_> {
    fn get_table(
        &self, 
        location: &Location
    ) -> &AnimationTable {
        let entity = *self.map.0.get(location).expect("Table entity not found for location");
        self.tables.get(entity).expect("ClientTable does not exist for table entity")
    }

    fn get_table_mut(
        &mut self, 
        location: &Location
    ) -> &mut AnimationTable {
        let entity = *self.map.0.get(location).expect("Table entity not found for location");
        self.tables.get_mut(entity).expect("ClientTable does not exist for table entity").into_inner()
    }
}

impl AnimationTracker<'_,'_> {
    pub fn enque_action(&mut self, action: DelayedAnimationAction) {
        self.animation_queue.queue.push_back(action)
    }

    fn transfer(
        &mut self,
        from: &CardReference,
        to: &CardReference,
        card: Option<Card>,
    ) {
        let mut item = self.remove(from).expect("Item did not exist");
        
        self.reset_mouse_offset(&item);
        self.set_sprite(&item, card);
        item.0 = card;

        self.push(to, item);
    }

    fn set_discard_last(&mut self, card: Option<Card>) {
        let discard = self.get_mut(
            &CardReference{
                location: Location::DiscardPile, 
                hand_position: HandPosition::Last
            }
        ).expect("No discarded card");
        
        discard.0 = card;
        let item = *discard;

        self.set_sprite(&item, card);
    }

    pub fn is_empty(&self) -> bool {
        self.animation_queue.queue.is_empty()
    }

    fn victory(&mut self, winner: usize) {
        println!("player with id {} won the game!", winner);
        self.commands.insert_resource(Victory{winner});
        self.commands.insert_resource(NextState(GameState::PostGame));
    }

    pub fn someone_has_two_cards(&mut self) {
        self.commands.init_resource::<CallDos>();
    }
}