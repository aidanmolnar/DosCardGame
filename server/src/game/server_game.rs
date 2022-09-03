use dos_shared::cards::{Card, CardType, CardColor};
use dos_shared::dos_game::{DosGame, DECK_REFERENCE};
use dos_shared::{table::*, GameInfo, GameState};
use dos_shared::transfer::{CardTransfer, Table};
use iyes_loopless::state::NextState;
use super::sync::ServerSyncer;
use super::table::ServerTable;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
pub struct ServerGame<'w,'s> {
    map: Res<'w, TableMap>,
    tables: Query<'w, 's, &'static mut ServerTable>,
    pub syncer: ResMut<'w, ServerSyncer>,
    commands: Commands<'w,'s>,

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
                self.syncer.add_card(player, card)
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
        let res = condition(self);
        self.syncer.add_condition(res);
        res
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

    fn reshuffle(&mut self) {
        while self.get_table(&Location::DiscardPile).len() > 1 {
            self.transfer(
                &CardReference { location: Location::DiscardPile, hand_position:HandPosition::Index(0)}, 
                &DECK_REFERENCE
            );

            // Reset wildcard colors
            let e = self.get_mut(&DECK_REFERENCE).unwrap();
            if e.ty == CardType::Wild || e.ty == CardType::DrawFour {
                e.color = CardColor::Wild;
            }
        }

        self.get_table_mut(&Location::Deck).shuffle();
    }

    fn victory(&mut self, winner: usize) {
        println!("player with id {} won the game!", winner);
        self.commands.insert_resource(NextState(GameState::MainMenu));
    }

    fn someone_has_two_cards(&mut self, player: usize) {
        println!("player with id {} has two cards!", player);
    }

}
