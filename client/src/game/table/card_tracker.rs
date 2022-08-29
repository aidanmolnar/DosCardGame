use dos_shared::GameInfo;
use dos_shared::dos_game::DosGame;
use dos_shared::table::{TableMap, Location, CardReference, HandPosition};
use dos_shared::transfer::{BasicTable, CardWrapper, Table};
use dos_shared::{transfer::CardTracker, cards::Card};

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use crate::multiplayer::MultiplayerState;

use super::animation_tracker::{AnimationTracker, DelayedAnimationAction, AnimationAction};
use super::memorized_cards::MemorizedCards;

#[derive(Component, Debug, Clone)]
pub struct ClientTable (BasicTable<ClientItem>);

impl ClientTable {
    pub fn new() -> Self {
        ClientTable (
            BasicTable(Vec::new())
        )
    }

    pub fn new_deck(num_cards: usize) -> Self {
        ClientTable (
            BasicTable(vec![ClientItem(None); num_cards])
        )
    }
}

impl Table<ClientItem> for ClientTable {
    fn remove(
        &mut self,
        index: usize
    ) -> Option<ClientItem> {
        self.0.remove(index)
    }

    fn push(
        &mut self,
        item: ClientItem
    ) {
        self.0.push(item)
    }

    fn last(
        &self,
    ) -> Option<&ClientItem> {
        self.0.last()
    }

    fn last_mut(
        &mut self,
    ) -> Option<&mut ClientItem> {
        self.0.last_mut()
    }

    fn len(
        &self
    ) -> usize {
        self.0.len()
    }

    fn pop(
        &mut self
    ) -> Option<ClientItem> {
        self.0.pop()
    }

    fn get(
        &self,
        index: usize,
    ) -> Option<&ClientItem> {
        self.0.get(index)
    }
    
    fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut ClientItem> {
        self.0.get_mut(index)
    }
}

#[derive(Debug, Clone)]
pub struct ClientItem (Option<Card>);

impl CardWrapper for ClientItem {
    fn card(&self) ->&Card {
        self.0.as_ref().expect("Card must exist")
    }

    fn card_mut(&mut self) -> &mut Card {
        self.0.as_mut().expect("Card must exist")
    }
}

#[derive(SystemParam)]
pub struct ClientCardTracker<'w,'s> {
    map: Res<'w, TableMap>,
    tables: Query<'w, 's, &'static mut ClientTable>,

    animation_tracker: AnimationTracker<'w,'s>,

    pub memorized_cards: ResMut<'w, MemorizedCards>,
    pub mp_state: ResMut<'w, MultiplayerState>,
    game_info: ResMut<'w, GameInfo>,
}

impl CardTracker<ClientItem, ClientTable> for ClientCardTracker<'_, '_> {
    fn get_table(
        & self, 
        location: &Location
    ) -> & ClientTable {
        let entity = *self.map.0.get(location).expect("Table entity not found for location");
        self.tables.get(entity).expect("Table does not exist for table entity")
    }

    fn get_table_mut(
        & mut self, 
        location: &Location
    ) -> & mut ClientTable {
        let entity = *self.map.0.get(location).expect("Table entity not found for location");
        self.tables.get_mut(entity).expect("Table does not exist for table entity").into_inner()
    }
}

impl ClientCardTracker<'_,'_> {
    fn get_card_value(
        &mut self, 
        from: &Location,
        to: &Location
    ) -> Option<Card> {
        if !self.is_visible(from, self.mp_state.turn_id) &&
        self.is_visible(to, self.mp_state.turn_id) {
            Some(self.memorized_cards.deque())
        } else {
            None
        }
    }
}

impl DosGame<ClientItem, ClientTable> for ClientCardTracker<'_,'_> {
    fn game_info(&self) -> &GameInfo {
        &self.game_info
    }

    fn game_info_mut(&mut self) -> &mut GameInfo {
        &mut self.game_info
    }

    fn server_condition<F>(&mut self, _condition: F) -> bool
    where F: Fn(&Self) -> bool {
        self.memorized_cards.1 -= 1;
        self.memorized_cards.1 == 0
    }

    fn set_discard_last(
        &mut self, 
        card: Option<Card>
    ) {
        let discard = self.get_mut(
            &CardReference{
                location: Location::DiscardPile, 
                hand_position: HandPosition::Last
            }
        ).expect("No discarded card");
        discard.0 = card;

        self.animation_tracker.enque_action(DelayedAnimationAction{
            action: AnimationAction::SetDiscardLast{card},
            delay: 0.1,
        });
    }

    fn transfer(
        &mut self,
        from: &dos_shared::table::CardReference,
        to: &dos_shared::table::CardReference,
    ) {
        let mut item = self.remove(from).expect("Item did not exist");

        let card = self.get_card_value(&from.location, &to.location);

        if item.0.is_none() {
            item.0 = card;
        }

        self.push(to, item);

        self.animation_tracker.enque_action(DelayedAnimationAction{
            action: AnimationAction::Transfer{from: *from, to: *to, card},
            delay: 0.1,
        });
    }
}

impl ClientCardTracker<'_,'_> {
    pub fn has_delayed_transfers(&self) -> bool {
        false
    }
}