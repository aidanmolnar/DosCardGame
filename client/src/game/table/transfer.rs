use dos_shared::cards::Card;
use dos_shared::table::*;

use crate::game::animations::components::*;
use crate::game::card_indexing::SpriteIndex;
use super::ClientTable;


use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

// A resource for handling moving cards from tavle to table
#[derive(SystemParam)]
pub struct CardTransferer<'w,'s> {
    commands: Commands<'w, 's>,
    map: Res<'w, TableMap>,
    tables: Query<'w, 's, &'static mut ClientTable>,
    sprites: Query<'w, 's, &'static mut TextureAtlasSprite>,
}

impl<'w,'s> CardTransferer<'w,'s> {
    fn find_table(
        &mut self,
        location: &Location,
    ) -> Mut<ClientTable> {
        let table_entity = self.map.0[location];
        self.tables.get_mut(table_entity).unwrap()
    }

    fn get (
        &mut self, 
        from: &CardReference
    ) -> Entity {
        self.find_table(&from.location).remove(from.index)
    }

    fn insert (
        &mut self, 
        to: &CardReference, 
        entity: Entity, 
        card: Option<Card>
    ) {
        self.find_table(&to.location).insert(card, entity);
    }

    fn modify (
        &mut self, 
        card: Option<Card>, 
        entity: Entity
    ) {
        // TODO: make this some sort of animation... maybe flip the card or something?
        if let Some(card) = card {
            let mut sprite = self.sprites.get_mut(entity).unwrap();
            sprite.index = card.get_sprite_index();
        }
    
        // TODO: may be unnecessary to clear mouse offset
        self.commands.entity(entity).insert(MouseOffset{offset: Vec3::ZERO, scale: 1.});
    }

    pub fn peek_discard(&mut self) -> Option<Card> {
        if let Some((_, card)) = self.find_table(&Location::DiscardPile).last() {
            card
        } else {
            None
        }
    }

    pub fn peek_staging(&mut self) -> Option<Card> {
        if let Some((_, card)) = self.find_table(&Location::Staging).last() {
            card
        } else {
            None
        }
    }

    pub fn set_discard_value(&mut self, card: Option<Card>) {
        let mut table = self.find_table(&Location::DiscardPile);
        let (entity, _) = table.last().unwrap();

        // Update the tracked value
        table.set_last_value(card);

        // Update the displayed value
        if let Some(card) = card {
            let mut sprite = self.sprites.get_mut(entity).unwrap();
            sprite.index = card.get_sprite_index();
        }
    }

    pub fn transfer (
        &mut self, 
        from: CardReference, 
        to: CardReference, 
        card: Option<Card>
    ) {
        let entity = self.get(&from);
        self.modify(card, entity);
        self.insert(&to, entity, card);
    }
}