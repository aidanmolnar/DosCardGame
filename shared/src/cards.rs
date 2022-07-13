use rand::thread_rng;
use rand::seq::SliceRandom;

use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub color: CardColor, //TODO: This doesn't have to be an option.. could be included in color
    pub ty: CardType,
}



#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]

pub enum CardColor {
    Red,
    Blue,
    Green,
    Yellow,
    Wild,
}


#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CardType {
    Basic(u8),
    Skip,
    Reverse,
    DrawTwo,
    Wild,
    DrawFour,
}

pub fn new_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    add_color_cards(&mut deck, CardColor::Red);
    add_color_cards(&mut deck, CardColor::Green);
    add_color_cards(&mut deck, CardColor::Blue);
    add_color_cards(&mut deck, CardColor::Yellow);

    for _ in 0..4 {
        deck.push(Card {
            color: CardColor::Wild,
            ty: CardType::Wild,
        });
        deck.push(Card {
            color: CardColor::Wild,
            ty: CardType::DrawFour,
        });
    }

    deck.shuffle(&mut thread_rng());

    deck
}

// Adds all the colored cards for one color
fn add_color_cards(deck: &mut Vec<Card>, color: CardColor) {
    // Add the basic numbered cards
    deck.push(Card{ color, ty: CardType::Basic(0) });
    for i in 1..=9 {
        deck.push(Card{ color, ty: CardType::Basic(i) });
        deck.push(Card{ color, ty: CardType::Basic(i) });
    }

    // Adds action cards
    deck.push(Card{ color, ty: CardType::Skip });
    deck.push(Card{ color, ty: CardType::Skip });
    deck.push(Card{ color, ty: CardType::Reverse });
    deck.push(Card{ color, ty: CardType::Reverse });
    deck.push(Card{ color, ty: CardType::DrawTwo });
    deck.push(Card{ color, ty: CardType::DrawTwo });
}
