
use super::cards::*;
use std::fmt;

pub struct Player<'a> {
    pub id: u8,
    pub hand: Vec<Card>,
    pub agent: &'a dyn Agent,
}

impl<'a> fmt::Debug for Player<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Player")
         .field("id", &self.id)
         .field("hand", &self.hand)
         .finish()
    }
}


pub trait Agent {

}

pub struct Bot {

}

impl Agent for Bot {

}