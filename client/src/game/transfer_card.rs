use bevy::ecs::system::SystemParam;
use dos_shared::cards::Card;
use dos_shared::table::*;

use crate::multiplayer::MultiplayerState;

use super::animations::components::MouseOffset;
use super::table::ClientTable;
use super::card_building::card_indexing::SpriteIndex;

use bevy::prelude::*;

#[derive(SystemParam)]
pub struct CardTransferer<'w,'s> {
    commands: Commands<'w, 's>,
    mp_state: Res<'w, MultiplayerState>,
    map: Res<'w, TableMap>,
    tables: Query<'w, 's, &'static mut ClientTable>,
    query: Query<'w, 's, &'static mut TextureAtlasSprite>,
}

impl<'w,'s> CardTransferer<'w,'s> {
    fn get_card(
        &mut self, 
        from: &CardReference
    ) -> Entity {
        let from_entity = self.map.0[&from.location];
        let mut from_table = self.tables.get_mut(from_entity).unwrap();
        from_table.remove(from.index)
    }

    fn insert_card(
        &mut self, 
        to: &CardReference, 
        entity: Entity, 
        card: Option<Card>
    ) {
        println!("{:?}", to.location);
        let to_entity = self.map.0[&to.location];
        
        let mut to_table = self.tables.get_mut(to_entity).unwrap();
        to_table.insert(entity, card);
    }

    fn modify_card(
        &mut self, 
        from: &CardReference, 
        to: &CardReference, 
        card: Option<Card>, 
        entity: Entity
    ) {
        if let Some(card) = card {
            let mut atlas = self.query.get_mut(entity).unwrap();
            atlas.index = card.get_sprite_index();
        }
    
        let your_hand_location = Location::Hand{player_id: self.mp_state.turn_id};
        if from.location == your_hand_location {
            self.commands.entity(entity).remove::<MouseOffset>();
        }  
        if to.location == your_hand_location {
            self.commands.entity(entity).insert(MouseOffset{offset: Vec3::ZERO, scale: 1.});
        }  
    }

    pub fn transfer_card(
        &mut self, 
        from: CardReference, 
        to: CardReference, 
        card: Option<Card>
    ) {
        let entity = self.get_card(&from);
        self.modify_card(&from, &to, card, entity);
        self.insert_card(&to, entity, card);
    }
}