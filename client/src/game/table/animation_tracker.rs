use dos_shared::cards::Card;
use dos_shared::table::{Location, CardReference, TableMap, HandPosition};
use dos_shared::transfer::CardTracker;

use super::animation_table::{AnimationItem, AnimationTable};


use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::game::animations::components::MouseOffset;
use crate::game::card_indexing::SpriteIndex;


#[derive(SystemParam)]
pub struct AnimationTracker<'w,'s> {
    map: Res<'w, TableMap>,
    tables: Query<'w, 's, &'static mut AnimationTable>,

    commands: Commands<'w, 's>,
    sprites: Query<'w, 's, &'static mut TextureAtlasSprite>,
}

impl AnimationTracker<'_,'_> {
    pub fn set_sprite(&mut self, item: &AnimationItem, new_card: Option<Card>) {
        if let Some(card) = new_card {
            let mut sprite = self.sprites.get_mut(item.1).unwrap();
            sprite.index = card.get_sprite_index();
        }
    }

    pub fn reset_mouse_offset(&mut self, item: &AnimationItem) {
        self.commands.entity(item.1).insert(MouseOffset{offset: Vec3::ZERO, scale: 1.});
    }
}

impl CardTracker<AnimationItem, AnimationTable> for AnimationTracker<'_, '_> {
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
    pub fn transfer(
        &mut self,
        from: &CardReference,
        to: &CardReference,
        card: Option<Card>,
    ) {
        let mut item = self.remove(from).expect("Item did not exist");
        
        self.reset_mouse_offset(&item);

        if item.0.is_none() {
            item.0 = card;
            self.set_sprite(&item, card); // TODO: could combine above and this line into set_card func
        }

        self.push(to, item);
    }

    pub fn set_discard_last(&mut self, card: Option<Card>) {
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
}