use dos_shared::cards::*;

use bevy::prelude::*;


#[derive(Default)]
pub struct CardTracker {
    pub your_cards: Vec<Card>,
    pub map: Vec<Vec<Entity>>,
}

impl CardTracker {
    pub fn add_card(
        &mut self,
        card: Option<Card>,
        card_entity: Entity,
        card_owner_id: u8,
        your_id: u8,
    ) {
        // Handles case where the card owner is the local player
        if card_owner_id == your_id {
            let hand_position = self.your_cards.binary_search_by(|x| x.cmp(&card.unwrap())).unwrap_or_else(|x| x);
            self.your_cards.insert(hand_position, card.unwrap());
    
            // Inserts into card tracker, maintaining sorted order by card value
            if let Some(vec) = self.map.get_mut(your_id as usize) {
                vec.insert(hand_position, card_entity);
            } else {
                self.map.push(vec![card_entity]);
            }
        // Handles case where the card owner is an opponent
        } else if let Some(vec) = self.map.get_mut(card_owner_id as usize) {
            vec.push(card_entity);
        } else {
            self.map.push(vec![card_entity]);
        }
        // TODO: Consider teammates?
    }

    // TODO: remove card
}