use dos_shared::cards::Card;

use dos_shared::transfer::{Table, BasicTable, CardWrapper};

use bevy::prelude::{Entity, Component};

// Client representation of a card for tables and trackers
#[derive(Copy, Clone, Debug)]
pub struct ClientItem(pub Option<Card>, pub Entity);

impl CardWrapper for ClientItem {
    fn card(&self) ->&Card {
        self.0.as_ref().expect("Card value unknown")
    }

    fn card_mut(&mut self) -> &mut Card {
        self.0.as_mut().expect("Card value unknown")
    }
}

#[derive(Debug, Clone)]
pub struct SortedTable {
    entities: BasicTable<Entity>, // Ordered by actual hand position
    cards: Vec<ClientItem>, // A vec where an internal ordering by card value is maintaned
}

impl SortedTable {
    fn sorted_index(&self, entity: Entity) -> Option<usize> {
        self.cards.iter()
        .map(|x|x.1)
        .position(|e| e == entity)
    }
}

// TODO: Goals: implement all required methods in a coherent way.  Reasonable efficiency
impl ClientTable {
    pub fn new_sorted() -> Self {
        ClientTable::Sorted(SortedTable {
            entities: BasicTable(Vec::new()),
            cards: Vec::new(),
        })
    }

    pub fn new_unsorted() -> Self {
        ClientTable::Unsorted(
            BasicTable(Vec::new())
        )
    }

    pub fn new_unsorted_with_items(items: Vec<ClientItem>) -> Self {
        ClientTable::Unsorted(
            BasicTable(items)
        )
    }

    // TODO: Doesn't distinguish between table not having sorted index (i.e. unsorted) and entity not being in table
    pub fn sorted_index(&self, entity: Entity) -> Option<usize> {
        match self {
            ClientTable::Sorted(table) => {
                table.sorted_index(entity)
            }
            ClientTable::Unsorted(_) => {
                None
            }
        }
    }

    pub fn actual_index(&self, entity: Entity) -> Option<usize> {
        match self {
            ClientTable::Sorted(table) => {
                table.entities.0.iter().position(|e| *e == entity)
            }
            ClientTable::Unsorted(table) => {
                table.0.iter().position(|e| e.1 == entity)
            }
        }
    }

    // TODO: Doesn't distinguish between card being unknown/face-down and card not being in table
    // NOTE: Commented out because it is dead code.  TODO: Decide on removal
    // pub fn last_card(&self) -> Option<Card> {
    //     match self {
    //         ClientTable::Sorted(table) => {
    //             if let Some(item) = table.last() {
    //                 item.0
    //             } else {
    //                 None
    //             }
    //         }
    //         ClientTable::Unsorted(table) => {
    //             if let Some(item) = table.0.last() {
    //                 item.0
    //             } else {
    //                 None
    //             }
    //         }
    //     }
    // }

    // TODO: Doesn't distinguish between card value being unknown/face-down and card not being in table
    pub fn card(&self, entity: Entity) -> Option<Card> {
        match self {
            ClientTable::Sorted(table) => {
                if let Some(index) = self.sorted_index(entity) {
                    table.cards[index].0
                } else {
                    None
                }
            }
            ClientTable::Unsorted(table) => {
                if let Some(index) = self.actual_index(entity) {
                    table.0[index].0
                } else {
                    None
                }
            }
        }
    }

    // TODO: Maybe consider just defining iter on each sub-type instead of requiring dynamic dispatch
    pub fn iter_entities(&'_ self) -> Box<dyn Iterator<Item = &Entity> + '_> {
        match self {
            ClientTable::Sorted(table) => {
                Box::new(table.cards.iter().map(|x| &x.1))
            }
            ClientTable::Unsorted(table) => {
                Box::new(table.0.iter().map(|x| &x.1))
            }
        }
    }
}

impl Table<ClientItem> for SortedTable {
    fn push(&mut self, item: ClientItem) {
        let hand_position = self.cards.binary_search_by(|x| x.0.cmp(&item.0)).unwrap_or_else(|x| x);
        self.cards.insert(hand_position, item);
        self.entities.push(item.1);
    }

    fn remove(&mut self, index: usize) -> Option<ClientItem> {
        if let Some(entity) = self.entities.remove(index) {
            // We know the entity has been inserted into the table if it was in entities
            Some(self.cards.remove(
                self.sorted_index(entity).unwrap()
            ))
        } else {
            None
        }
    }

    fn last(&self) -> Option<&ClientItem> {
        if let Some(entity) = self.entities.last() {
            // We know the entity has been inserted into the table if it was in entities
            Some(&self.cards[self.sorted_index(*entity).unwrap()])
        } else {
            None
        }
    }

    fn last_mut(
        &mut self,
    ) -> Option<&mut ClientItem> {
        panic!("Can't mutate sorted table.")
    }

    fn len(
        &self
    ) -> usize {
        self.entities.len()
    }

    fn pop(
        &mut self
    ) -> Option<ClientItem> {
        if let Some(entity) = self.entities.pop() {
            // We know the entity has been inserted into the table if it was in entities
            Some(self.cards.remove(
                self.sorted_index(entity).unwrap()
            ))
        } else {
            None
        }
    }

    fn get(
        &self,
        index: usize,
    ) -> Option<&ClientItem> {
        if let Some(entity) = self.entities.get(index) {
            // We know the entity has been inserted into the table if it was in entities
            Some(&self.cards[
                self.sorted_index(*entity).unwrap()
            ])
        } else {
            None
        }
    }

    fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut ClientItem> {
        if let Some(entity) = self.entities.get(index) {
            // We know the entity has been inserted into the table if it was in entities
            let index =  self.sorted_index(*entity).unwrap();
            Some(&mut self.cards[
               index
            ])
        } else {
            None
        }
    }

}

#[derive(Component, Debug, Clone)]
pub enum ClientTable {
    Sorted(SortedTable),
    Unsorted(BasicTable<ClientItem>),
}

impl Table<ClientItem> for ClientTable {
    fn remove(
        &mut self,
        index: usize
    ) -> Option<ClientItem> {
        match self {
            ClientTable::Sorted(table) => {table.remove(index)}
            ClientTable::Unsorted(table) => {table.remove(index)}
        }
    }

    fn push(
        &mut self,
        item: ClientItem
    ) {
        match self {
            ClientTable::Sorted(table) => {table.push(item)}
            ClientTable::Unsorted(table) => {table.push(item)}
        }
    }

    fn last(
        &self,
    ) -> Option<&ClientItem> {
        match self {
            ClientTable::Sorted(table) => {table.last()}
            ClientTable::Unsorted(table) => {table.last()}
        }
    }

    fn last_mut(
        &mut self,
    ) -> Option<&mut ClientItem> {
        match self {
            ClientTable::Sorted(table) => {table.last_mut()}
            ClientTable::Unsorted(table) => {table.last_mut()}
        }
    }

    fn len(
        &self
    ) -> usize {
        match self {
            ClientTable::Sorted(table) => {table.len()}
            ClientTable::Unsorted(table) => {table.len()}
        }
    }

    fn pop(
        &mut self
    ) -> Option<ClientItem> {
        match self {
            ClientTable::Sorted(table) => {table.pop()}
            ClientTable::Unsorted(table) => {table.pop()}
        }
    }

    fn get(
        &self,
        index: usize,
    ) -> Option<&ClientItem> {
        match self {
            ClientTable::Sorted(table) => {table.get(index)}
            ClientTable::Unsorted(table) => {table.get(index)}
        }
    }

    fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut ClientItem> {
        match self {
            ClientTable::Sorted(table) => {table.get_mut(index)}
            ClientTable::Unsorted(table) => {table.get_mut(index)}
        }
    }
}