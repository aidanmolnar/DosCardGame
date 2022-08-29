use dos_shared::cards::Card;
use dos_shared::dos_game::DosGame;
use dos_shared::{table::*, GameInfo};
use dos_shared::transfer::CardTransfer;
use super::sync::ServerSyncer;
use super::table::ServerTable;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;


#[derive(SystemParam)]
pub struct ServerGame<'w,'s> {
    map: Res<'w, TableMap>,
    tables: Query<'w, 's, &'static mut ServerTable>,
    pub syncer: ResMut<'w, ServerSyncer>,

    game_info: ResMut<'w, GameInfo>,
}

impl ServerGame<'_,'_> {
    fn record_card_value(
        &mut self, 
        from: &Location,
        to: &Location,
        card: Card
    ) {
        for player in 0..self.game_info.num_players() {
            if !self.is_visible(from, player) &&
            self.is_visible(to, player) {
                self.syncer.add(player, card)
            }
        }
    }
}

impl CardTransfer<Card, ServerTable> for ServerGame<'_, '_> {
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

}

impl DosGame<Card, ServerTable> for ServerGame<'_,'_> {
    fn game_info(&self) -> &GameInfo {
        &self.game_info
    }

    fn game_info_mut(&mut self) -> &mut GameInfo {
       &mut self.game_info
    }

    fn server_condition<F>(&mut self, condition: F) -> bool
    where F: Fn(&Self) -> bool {
        self.syncer.increment_condition_counter();
        condition(self)
    }

    fn set_discard_last(&mut self, card: Option<Card>) {
        let discard = self.get_mut(
            &CardReference{
                location: Location::DiscardPile, 
                hand_position: HandPosition::Last
            }
        ).expect("No discarded card");
        *discard = card.expect("Cards on server must have known value");
    }

    fn transfer(
        & mut self,
        from: &CardReference,
        to: &CardReference,
    ) {
        let card = self.remove(from).expect("Card did not exist");
        
        self.record_card_value(&from.location, &to.location, card);

        self.push(to, card);
    }
}
