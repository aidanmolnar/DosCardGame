
use super::table::{CardReference, Location, HandPosition, Table};

// Methods for mapping a card reference to a card.
// T is the type that represents a card.
pub trait CardTransfer<T, U: Table<T> + 'static> {
    fn get_table(
        &self, 
        location: &Location
    ) -> & U;

    fn get_table_mut(
        &mut self, 
        location: &Location
    ) -> &mut U;
    
    fn remove(
        &mut self, 
        from: &CardReference
    ) -> Option<T> {
        let table = self.get_table_mut(&from.location);
        match from.hand_position {
            HandPosition::Last => {table.pop()}
            HandPosition::Index(i) => {table.remove(i)}
        }
    }

    fn get(
        &self, 
        from: &CardReference
    ) -> Option<&T> {
        let table = self.get_table(&from.location);
        match from.hand_position {
            HandPosition::Last => {table.last()}
            HandPosition::Index(i) => {table.get(i)}
        }
    }

    fn get_mut(
        &mut self, 
        from: &CardReference
    ) -> Option<&mut T> {
        let table = self.get_table_mut(&from.location);
        match from.hand_position {
            HandPosition::Last => {table.last_mut()}
            HandPosition::Index(i) => {table.get_mut(i)}
        }
    }

    fn push(
        & mut self,
        to: &CardReference,
        item: T,
    ) {
        self.get_table_mut(&to.location).push(item);
    }
}