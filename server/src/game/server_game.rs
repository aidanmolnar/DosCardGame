use dos_shared::{
    messages::lobby::{TableSnapshot, GameSnapshot},
    cards::{Card, CardType, CardColor}, 
    dos_game::{DosGame, DECK_REFERENCE}, 
    table::{CardReference, HandPosition, Location, Table}, 
    table_map::TableMap, 
    transfer::CardTransfer, 
    GameInfo, 
    GameState
};

use crate::game::call_dos::CallDos;

use super::{sync::ServerSyncer, table::ServerTable};

use bevy::{
    prelude::*, 
    utils::HashMap
};
use bevy::ecs::system::SystemParam;
use iyes_loopless::state::NextState;

#[derive(SystemParam)]
pub struct ServerGame<'w,'s> {
    map: Res<'w, TableMap>,
    tables: Query<'w, 's, &'static mut ServerTable>,
    pub syncer: ResMut<'w, ServerSyncer>,
    pub commands: Commands<'w,'s>,
    pub call_dos: Option<ResMut<'w, CallDos>>,

    game_info: ResMut<'w, GameInfo>,
}

impl ServerGame<'_,'_> {
    // Caches the card value if it will become visible to the player
    // Values are extracted and sent to player when sending game message
    fn record_card_value(
        &mut self, 
        from: &Location,
        to: &Location,
        card: Card
    ) {
        for player in 0..self.game_info.num_players() {
            if !self.is_visible(from, player) &&
            self.is_visible(to, player) {
                self.syncer.add_card(player, card);
            }
        }
    }

    // Generates a complete snapshot of the entire state of the game for reconnecting a player
    pub fn get_snapshot(&self, player: usize) -> GameSnapshot {
        let mut tables = HashMap::new();

        for (location, _) in &self.map.0 {
            let table = self.get_table(location);

            let data = if self.is_visible(location, player) {
                TableSnapshot::Known(table.cards())
            } else {
                TableSnapshot::Unknown(table.len())
            };

            tables.insert(*location, data);
        }

        GameSnapshot {
            tables,
            game_info: self.game_info.clone(),
            dos: self.call_dos.as_ref().map(|call| call.player),
        }
    }
}

impl CardTransfer<Card, ServerTable> for ServerGame<'_, '_> {
    fn get_table(
        & self, 
        location: &Location
    ) -> & ServerTable {
        let entity = *self.map.0.get(location).expect("Table entity not found for location");
        self.tables.get(entity).expect("Table does not exist on table entity")
    }

    fn get_table_mut(
        & mut self, 
        location: &Location
    ) -> & mut ServerTable {
        let entity = *self.map.0.get(location).expect("Table entity not found for location");
        self.tables.get_mut(entity).expect("Table does not exist on table entity").into_inner()
    }

}

impl DosGame<Card, ServerTable> for ServerGame<'_,'_> {
    fn game_info(&self) -> &GameInfo {
        &self.game_info
    }

    fn game_info_mut(&mut self) -> &mut GameInfo {
       &mut self.game_info
    }

    // Tracks conditions that depend on card values that are not visible to players
    // Conditions are extracted and sent to player when sending game message
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
        self.commands.insert_resource(NextState(GameState::PostGame));
    }

    fn someone_has_two_cards(&mut self, player: usize) {
        println!("player with id {} has two cards!", player);
        self.commands.insert_resource(
            CallDos {
                player,
                caller: None,
                graceperiod: None,
            }
        );
    }

}
