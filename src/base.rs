use rand::thread_rng;
use rand::seq::SliceRandom;


#[derive(Debug)]
pub struct DosGame {
    deck: Vec<Card>,
    discard_pile: Vec<Card>,
    players: Vec<Player>,
    current_turn: u8,
}


// Need a server representation and a client represention...
// What should be shared vs distinct?
impl DosGame {

    pub fn turn(&mut self) {

    }



    pub fn deal_in_players(&mut self) {
        for _ in 0..7 {
            for pid in 0..self.players.len( ) {
                self.deal_card(pid);
            }
        }

        // Give notice to player to start turn? return some sort of struct?
    }

    pub fn deal_card(&mut self, player_id: usize) {
        let card = self.draw_card();
        self.players[player_id].hand.push(card);
    }


    fn draw_card(&mut self) -> Card {

        if let Some(card) = self.deck.pop() {
            card
        } else {
            self.reshuffle_discard_pile();

            if let Some(card) = self.deck.pop() {
                card
            } else {
                // TODO: Handle this case gracefully, maybe return an option instead
                panic!("No cards left")
            }
        }
    }

    pub fn reshuffle_discard_pile(&mut self) {
        self.deck = self.discard_pile.clone();
        self.discard_pile = Vec::new();

        self.deck.shuffle(&mut thread_rng());
    }
}

// Should players be initialized and dealt cards?  yes?
pub fn new_game(num_players: u8) -> DosGame {

    let deck = new_deck();

    let mut players = Vec::new();
    for i in 0..num_players {
        players.push(Player {
            id: i,
            hand: Vec::new(),
        })
    }

    DosGame {
        deck,
        discard_pile: Vec::new(),
        players,
        current_turn: 0,
    }
}

fn new_deck() -> Vec<Card> {
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

#[derive(Debug)]
pub struct Player {
    id: u8,
    hand: Vec<Card>,
}

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