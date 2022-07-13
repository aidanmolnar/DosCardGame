use bevy::prelude::*;

// Tracks the state of entities representing drawn cards
#[derive(Default)]
pub struct AnimationEntityTracker {
    pub discard_pile: Vec<Entity>,
    pub hands: Vec<HandOfCards>,

    pub focused_card: Option<Entity>,
}   

pub struct HandOfCards (pub Vec<Entity>);

impl AnimationEntityTracker {
    // Adds a card to a players hand
    pub fn deal_card(
        &mut self, 
        player_id: usize, 
        hand_position: Option<usize>, 
        entity: Entity
    ) {
        let hand = self.hands.get_mut(player_id).expect("Invalid owner id");
        if let Some(hand_position) = hand_position {
            hand.0.insert(hand_position, entity);
        } else {
            hand.0.push(entity);
        }
    }

    // Transfers a card from a player's hand to the discard pile
    // TODO: this should return the entity
    pub fn play_card(
        &mut self, 
        player_id: usize, 
        hand_position: usize
    ) -> Entity {
        let hand = self.hands.get_mut(player_id).expect("Invalid owner id");

        if hand_position >= hand.0.len() {
            panic!("Invalid hand position {hand_position} for player {player_id}")
        }

        let entity = hand.0.remove(hand_position);
        self.discard_card(entity);

        entity
    }

    // Adds a card to the discard pile
    // TODO: this should return the entity
    pub fn discard_card(
        &mut self,
        entity: Entity,
    ) {
        self.discard_pile.push(entity);
    }

    // Clears the discard pile for reshuffling
    pub fn clear_discard_pile(
        &mut self
    ) {
        self.discard_pile.clear();
    }
}