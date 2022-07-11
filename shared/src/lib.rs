pub mod cards;
pub mod messages;

pub const NUM_STARTING_CARDS: u8 = 20;
pub const DEFAULT_IP: &str = "localhost:3333";


pub fn deal_cards<F: FnMut(usize)>(
    num_players: usize, 
    deck_size: usize,
    mut f: F,
) {
    let mut count = 0;

    for _ in 0..NUM_STARTING_CARDS {
        for player_id in 0..num_players {
            f(player_id); // do something

            // Exit before dealing last card so that it can be used for discard pile
            if count >= deck_size - 1 {
                return
            }
            count += 1;
        }
    }
}