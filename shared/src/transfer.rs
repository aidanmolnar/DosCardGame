use super::table::{Location, CardReference, HandPosition};
use super::cards::Card;

pub trait Table<T> {
    fn remove(
        &mut self,
        index: usize
    ) -> Option<T>;

    fn get(
        &self,
        index: usize,
    ) -> Option<&T>;

    fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut T>;

    fn push(
        &mut self,
        item: T
    );

    fn last(
        &self,
    ) -> Option<&T>;

    fn last_mut(
        &mut self,
    ) -> Option<&mut T>;

    fn len(
        &self
    ) -> usize;

    fn is_empty(
        &self
    ) -> bool {
        self.len() == 0
    }

    fn pop(
        &mut self
    ) -> Option<T>;
}

pub trait CardWrapper {
    fn card(&self) ->&Card;
    fn card_mut(&mut self) -> &mut Card;
}

impl CardWrapper for Card {
    fn card(&self) ->&Card {
        self
    }

    fn card_mut(&mut self) -> &mut Card {
        self
    }
}

pub trait CardTracker<T: CardWrapper, U: Table<T> + std::fmt::Debug + 'static> {
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
        self.get_table_mut(&to.location).push(item)
    }
}

#[derive(Debug, Clone)]
pub struct BasicTable<T> (pub Vec<T>);

impl<T: std::fmt::Debug> Table<T> for BasicTable<T> {
    fn remove(
        &mut self,
        index: usize
    ) -> Option<T> {
        if index < self.0.len() {
            Some(self.0.remove(index))
        } else {
            None
        }
    }

    fn get(
        &self,
        index: usize,
    ) -> Option<&T> {
        self.0.get(index)
    }

    fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut T> {
        self.0.get_mut(index)
    }

    fn push(
        &mut self,
        item: T
    ) {
        self.0.push(item)
    }

    fn last(
        &self,
    ) -> Option<&T> {
        self.0.last()
    }

    fn last_mut(
        &mut self,
    ) -> Option<&mut T> {
        self.0.last_mut()
    }

    fn len(
        &self
    ) -> usize {
        self.0.len()
    }

    fn pop(
        &mut self
    ) -> Option<T> {
        self.0.pop()
    }
}