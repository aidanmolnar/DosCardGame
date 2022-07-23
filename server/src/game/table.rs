use dos_shared::cards::Card;
use dos_shared::table::*;

use bevy::prelude::*;

#[derive(Component)]
struct ServerTable {
    cards: Vec<Card>
}

impl ServerTable {

    fn insert(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn remove(&mut self, index: Option<usize>) -> Card {
        if let Some(index) = index {
            self.cards.remove(index)
        } else {
            self.cards.pop().expect("No cards left")
        }
    }

    fn reshuffle(&mut self) {
        todo!();
    }
}

// This will be very similar on both client and server :)... :(
// Well no because on client card will need to be intercepted and have additional data applied to it? :/ This function might need to take a closure or something that operates on the card
fn transfer_card (
    from: CardReference,
    to: CardReference,
    map: ResMut<TableMap>,
    mut tables: Query<&mut ServerTable>,
) {
    let from_entity = map.0[&from.location];
    let mut from_table = tables.get_mut(from_entity).unwrap();
    let card = from_table.remove(from.index);

    let to_entity = map.0[&to.location];
    let mut to_table = tables.get_mut(to_entity).unwrap();
    to_table.insert(card);
}

