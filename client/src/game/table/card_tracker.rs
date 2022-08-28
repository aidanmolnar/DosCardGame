use dos_shared::cards::Card;
use dos_shared::dos_game::DosGame;
use dos_shared::table::{Location, CardReference, TableMap};
use dos_shared::transfer::{CardTracker, CardWrapper};
use dos_shared::GameInfo;

use super::client_table::{ClientItem, ClientTable};

use std::collections::VecDeque;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;



// TODO: we need to init this
#[derive(Default, Debug)]
pub struct MemorizedCards(VecDeque<Card>, pub usize);

impl MemorizedCards {
    pub fn enque(&mut self, card: Card) {
        self.0.push_back(card);
    }

    fn deque(&mut self) -> Card {
        self.0.pop_front().expect("No more card values stored")
    }
}

#[derive(SystemParam)]
pub struct ClientCardTracker<'w,'s> {
    commands: Commands<'w, 's>,
    map: Res<'w, TableMap>,
    tables: Query<'w, 's, &'static mut ClientTable>,
    sprites: Query<'w, 's, &'static mut TextureAtlasSprite>,
    pub memorized_cards: ResMut<'w, MemorizedCards>,
    delayed_transfer_queue: ResMut<'w, DelayedTransferQueue>,
    time: Res<'w, Time>,

    // TODO: Maybe better way to store this info... 
    // Shouldn't need whole mp_state to get your turn id
    pub mp_state: ResMut<'w, MultiplayerState>,
    game_info: ResMut<'w, GameInfo>,
}

use crate::game::animations::components::MouseOffset;
use crate::game::card_indexing::SpriteIndex;
use crate::multiplayer::MultiplayerState;
use dos_shared::transfer::is_visible;

impl ClientCardTracker<'_,'_> {
    fn set_sprite(&mut self, item: &ClientItem, new_card: Option<Card>) {
        if let Some(card) = new_card {
            let mut sprite = self.sprites.get_mut(item.1).unwrap();
            sprite.index = card.get_sprite_index();
        }
    }

    fn reset_mouse_offset(&mut self, item: &ClientItem) {
        self.commands.entity(item.1).insert(MouseOffset{offset: Vec3::ZERO, scale: 1.});
    }

    fn get_card_value(
        &mut self, 
        from: &Location,
        to: &Location
    ) -> Option<Card> {
        if !is_visible(from, self.mp_state.turn_id, self.game_info.current_turn()) &&
        is_visible(to, self.mp_state.turn_id, self.game_info.current_turn()) {
            Some(self.memorized_cards.deque())
        } else {
            None
        }
    }
}

impl CardTracker<ClientItem, ClientTable> for ClientCardTracker<'_, '_> {
    fn get_table(
        &self, 
        location: &Location
    ) -> &ClientTable {
        let entity = *self.map.0.get(location).expect("Table entity not found for location");
        self.tables.get(entity).expect("ClientTable does not exist for table entity")
    }

    fn get_table_mut(
        &mut self, 
        location: &Location
    ) -> &mut ClientTable {
        let entity = *self.map.0.get(location).expect("Table entity not found for location");
        self.tables.get_mut(entity).expect("ClientTable does not exist for table entity").into_inner()
    }

    fn transfer(
        &mut self,
        from: &CardReference,
        to: &CardReference,
    ) -> Option<Card> {
        let card = self.get_card_value(&from.location, &to.location);

        self.delayed_transfer_queue.add(DelayedTransfer {
            from: *from,
            to: *to,
            card,
            delay: 0.05,
        });
        

        if is_visible(&from.location, self.mp_state.turn_id, self.game_info.current_turn()) {
            Some(*self.get(from).unwrap().card())
        } else {
            card
        }
    }

    fn set_discard_last(&mut self, card: Option<Card>) {
        let discard = self.discard_last_mut().expect("No discarded card");
        discard.0 = card;
        let item = *discard;

        self.set_sprite(&item, card);
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
}

pub struct DelayedTransferQueue {
    transfers: VecDeque<DelayedTransfer>,
    timer: Timer,
}

struct DelayedTransfer {
    card: Option<Card>,
    from: CardReference,
    to: CardReference,
    delay: f32
}

use bevy::utils::Duration;

impl DelayedTransferQueue {
    fn update(&mut self, delta: Duration) -> Option<(Option<Card>, CardReference, CardReference)> {
        self.timer.tick(delta);

        if self.timer.finished() {
            if let Some(transfer) = self.transfers.pop_front() {
                self.timer = Timer::from_seconds(transfer.delay, false);
                Some((transfer.card, transfer.from, transfer.to))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn add(&mut self, transfer: DelayedTransfer) {
        self.transfers.push_back(transfer);
    }

    fn new() -> Self {
        DelayedTransferQueue {
            transfers: VecDeque::new(),
            timer: Timer::from_seconds(0.1, true)
        }
    }

}

impl ClientCardTracker<'_,'_> {
    fn process_transfer(
        &mut self,
        card: Option<Card>,
        from: &CardReference,
        to: &CardReference, 
    ) {
        let mut item = self.remove(from).expect("Item did not exist");
        dbg!(item);
        
        // TODO: may be unnecessary to clear mouse offset
        self.reset_mouse_offset(&item);

        //let card = self.get_card_value(&from.location, &to.location);

        if item.0.is_none() {
            item.0 = card;
            self.set_sprite(&item, card); // TODO: could combine above and this line into set_card func
        }

        self.push(to, item);
    }

    fn update(&mut self) {
        if let Some((card, from, to, )) = self.delayed_transfer_queue.update(self.time.delta()) {
            self.process_transfer(card,&from, &to);
        }
    }

    pub fn has_delayed_transfers(&self) -> bool{
        !self.delayed_transfer_queue.transfers.is_empty()
    }
}

pub fn setup_delayed_transfer_queue(
    mut commands: Commands
) {
    commands.insert_resource(DelayedTransferQueue::new());
}

// TODO: add this system so that enqueued transfers are processed
pub fn update_card_tracker_system(
    mut card_tracker: ClientCardTracker,
) {
    card_tracker.update()
}