use dos_shared::GameInfo;
use dos_shared::dos_game::{DosGame, DECK_REFERENCE};
use dos_shared::table::{TableMap, Location, CardReference, HandPosition};
use dos_shared::transfer::Table;
use dos_shared::{transfer::CardTransfer, cards::Card};

use crate::multiplayer::MultiplayerState;
use super::graphics::{AnimationTracker, DelayedAnimationAction, AnimationAction};
use super::sync::ClientSyncer;
use super::table::{ClientTable, ClientItem};

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
pub struct ClientGame<'w,'s> {
    map: Res<'w, TableMap>,
    tables: Query<'w, 's, &'static mut ClientTable>,

    animation_tracker: AnimationTracker<'w,'s>,

    pub syncer: ResMut<'w, ClientSyncer>,
    pub mp_state: ResMut<'w, MultiplayerState>,
    game_info: ResMut<'w, GameInfo>,
}

impl CardTransfer<ClientItem, ClientTable> for ClientGame<'_, '_> {
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

impl DosGame<ClientItem, ClientTable> for ClientGame<'_,'_> {
    fn game_info(&self) -> &GameInfo {
        &self.game_info
    }

    fn game_info_mut(&mut self) -> &mut GameInfo {
        &mut self.game_info
    }

    fn server_condition<F>(&mut self, _condition: F) -> bool
    where F: Fn(&Self) -> bool {
        self.syncer.deque_condition()
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

        // Get card value based on visibility rules
        let card = if self.is_visible(&from.location, self.mp_state.turn_id) {
            if self.is_visible(&to.location, self.mp_state.turn_id) {
                // Do nothing
                item.0
            } else {
                // Set to None
                None
            }
        } else {
            #[allow(clippy::collapsible_else_if)] // Makes it more readable
            if self.is_visible(&to.location, self.mp_state.turn_id) {
                // Get the value
                Some(self.syncer.deque_card())
            } else {
                // Set to None
                None
            }
        };
        item.0 = card;

        self.push(to, item);

        self.animation_tracker.enque_action(DelayedAnimationAction{
            action: AnimationAction::Transfer{from: *from, to: *to, card},
            delay: 0.1,
        });
    }

    fn reshuffle(&mut self) {
        while self.get_table(&Location::DiscardPile).len() > 1 {
            self.transfer(
                &CardReference { location: Location::DiscardPile, hand_position:HandPosition::Index(0)}, 
                &DECK_REFERENCE
            );
        }
    }

    // TODO: these need to be enqueed
    fn victory(&mut self, winner: usize) {
        
        self.animation_tracker.enque_action(DelayedAnimationAction{ action: AnimationAction::Victory{winner}, delay: 0.5 })
    }

    fn someone_has_two_cards(&mut self, player: usize) {
        println!("player with id {} has two cards!", player);
    }

}

impl ClientGame<'_,'_> {
    pub fn has_delayed_transfers(&self) -> bool {
        !self.animation_tracker.is_empty()
    }
}