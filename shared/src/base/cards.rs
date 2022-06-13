use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Copy, Clone, Debug)]
pub enum Card {
    Basic {
        color: Color,
        value: u8,
    },
    Reverse {
        color: Color,
    },
    Skip {
        color: Color,
    },
    DrawTwo {
        color: Color,
    },
    DrawFour {
    },
    Wild {
    },
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

pub fn new_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    add_color_cards(&mut deck, Color::Red);
    add_color_cards(&mut deck, Color::Green);
    add_color_cards(&mut deck, Color::Blue);
    add_color_cards(&mut deck, Color::Yellow);

    for _ in 0..4 {
        deck.push(Card::Wild{});
        deck.push(Card::DrawFour{});
    }

    deck.shuffle(&mut thread_rng());

    deck
}

// Adds all the colored cards for one color
fn add_color_cards(deck: &mut Vec<Card>, color: Color) {
    // Add the basic numbered cards
    deck.push(Card::Basic { color: Color::Red, value: 0 });
    for i in 1..=9 {
        deck.push(Card::Basic { color, value: i });
        deck.push(Card::Basic { color, value: i });
    }

    // Adds action cards
    deck.push(Card::Reverse{color});
    deck.push(Card::Reverse{color});
    deck.push(Card::Skip{color});
    deck.push(Card::Skip{color});
    deck.push(Card::DrawTwo{color});
    deck.push(Card::DrawTwo{color});
}
