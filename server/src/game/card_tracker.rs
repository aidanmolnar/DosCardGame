use dos_shared::cards::Card;
use dos_shared::dos_game::DosGame;
use dos_shared::{table::*, GameInfo};
use dos_shared::transfer::{BasicTable, Table, CardTracker};

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

#[derive(Component, Debug)]
pub struct ServerTable(BasicTable<Card>);

impl ServerTable {
    pub fn new(cards: Vec<Card>) -> Self {
        ServerTable(
            BasicTable::<Card>(cards)
        )
    }
}

impl Default for ServerTable {
    fn default() -> Self {
        ServerTable(
            BasicTable::<Card>(Vec::new())
        )
    }
}

impl Table<Card> for ServerTable {
    fn remove(
        &mut self,
        index: usize
    ) -> Option<Card> {
        self.0.remove(index)
    }

    fn push(
        &mut self,
        item: Card
    ) {
        self.0.push(item)
    }

    fn last(
        &self,
    ) -> Option<&Card> {
        self.0.last()
    }

    fn last_mut(
        &mut self,
    ) -> Option<&mut Card> {
        self.0.last_mut()
    }

    fn len(
        &self
    ) -> usize {
        self.0.len()
    }

    fn pop(
        &mut self
    ) -> Option<Card> {
        self.0.pop()
    }

    fn get(
        &self,
        index: usize,
    ) -> Option<&Card> {
        self.0.get(index)
    }
}

#[derive(SystemParam)]
pub struct ServerCardTracker<'w,'s> {
    map: Res<'w, TableMap>,
    tables: Query<'w, 's, &'static mut ServerTable>,
    pub memorized_cards: ResMut<'w, MemorizedCards>,

    game_info: ResMut<'w, GameInfo>,
}

use dos_shared::transfer::is_visible;

use std::mem;

use crate::multiplayer::AgentTracker;

pub struct MemorizedCards (Vec<Vec<Card>>, usize);

impl MemorizedCards {
    fn new(num_players: usize) -> Self {
        let mut players = Vec::with_capacity(num_players);
        for _ in 0..num_players {
            players.push(Vec::new())
        }
        MemorizedCards(players, 0)
    }

    fn add(&mut self, player: usize, card: Card) {
        self.0[player].push(card);
    }

    pub fn take_player(&mut self, player: usize) -> Vec<Card>{
        mem::take(&mut self.0[player])
    }

    fn increment_condition_counter(&mut self) {
        self.1 += 1;
    }

    pub fn take_condition_counter(&mut self) -> usize {
        std::mem::take(&mut self.1)
    }
}

pub fn setup_memorized_cards(
    agent_tracker: Res<AgentTracker>,
    mut commands: Commands,
) {
    commands.insert_resource(MemorizedCards::new(agent_tracker.agents.len()))
}

impl ServerCardTracker<'_,'_> {
    fn record_card_value(
        &mut self, 
        from: &Location,
        to: &Location,
        card: Card
    ) {
        for player in 0..self.game_info.num_players() {
            if !is_visible(from, player, self.game_info.current_turn()) &&
            is_visible(to, player, self.game_info.current_turn()) {
                self.memorized_cards.add(player, card)
            }
        }
    }
}

impl CardTracker<Card, ServerTable> for ServerCardTracker<'_, '_> {
    fn get_table(
        & self, 
        location: &Location
    ) -> & ServerTable {
        let entity = *self.map.0.get(location).expect("Table entity not found for location");
        self.tables.get(entity).expect("Table does not exist for table entity")
    }

    fn get_table_mut(
        & mut self, 
        location: &Location
    ) -> & mut ServerTable {
        let entity = *self.map.0.get(location).expect("Table entity not found for location");
        self.tables.get_mut(entity).expect("Table does not exist for table entity").into_inner()
    }

    fn set_discard_last(&mut self, card: Option<Card>) {
        let discard = self.discard_last_mut().expect("No discarded card");
        *discard = card.expect("Cards on server must have known value");
    }

    fn transfer(
        & mut self,
        from: &CardReference,
        to: &CardReference,
    ) -> Option<Card> {
        let card = self.remove(from).expect("Card did not exist");
        
        self.record_card_value(&from.location, &to.location, card);

        self.push(to, card);

        Some(card)
    }
}

impl DosGame<Card, ServerTable> for ServerCardTracker<'_,'_> {
    fn game_info(&self) -> &GameInfo {
        &self.game_info
    }

    fn game_info_mut(&mut self) -> &mut GameInfo {
       &mut self.game_info
    }

    fn server_condition<F>(&mut self, condition: F) -> bool
    where F: Fn(&Self) -> bool {
        self.memorized_cards.increment_condition_counter();
        condition(self)
    }
}