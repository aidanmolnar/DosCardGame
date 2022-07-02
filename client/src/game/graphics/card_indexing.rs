use dos_shared::cards::*;

pub const CARD_BACK_SPRITE_INDEX: usize = 4*13+2;

// Get the index of the card from the sprite sheet
pub fn get_index(card: &Card) -> usize {
    let offset = match card.color {
        CardColor::Red    => {   0}
        CardColor::Yellow => {  13}
        CardColor::Green  => {2*13}
        CardColor::Blue   => {3*13}
        CardColor::Wild   => {4*13} 
    };

    offset + match card.ty {
        CardType::Basic(i) => {i as usize}
        CardType::Skip =>     {10}
        CardType::Reverse =>  {11}
        CardType::DrawTwo =>  {12}
        CardType::Wild =>     {0}
        CardType::DrawFour => {1}
    }
}