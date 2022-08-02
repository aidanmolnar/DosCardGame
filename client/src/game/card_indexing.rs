use dos_shared::cards::*;

pub const CARD_BACK_SPRITE_INDEX: usize = 4*13+2;

// TODO: this could be moved to graphic_interface/assets?

// Get the index of the card from the sprite sheet

pub trait SpriteIndex {
    fn get_sprite_index(&self) -> usize;
}

impl SpriteIndex for Card {
    fn get_sprite_index(&self) -> usize {
        let offset = match self.color {
            CardColor::Red    => {   0}
            CardColor::Yellow => {  13}
            CardColor::Green  => {2*13}
            CardColor::Blue   => {3*13}
            CardColor::Wild   => {4*13} 
        };
    
        offset + match self.ty {
            CardType::Basic(i) => {i as usize}
            CardType::Skip =>     {10}
            CardType::Reverse =>  {11}
            CardType::DrawTwo =>  {12}
            CardType::Wild =>     {0}
            CardType::DrawFour => {1}
        }
    }
}

impl SpriteIndex for Option<Card> {
    fn get_sprite_index(&self) -> usize {

        // Get sprite info
        if let Some(card_value) = self {
            // TODO: move this to shared or implement as trait on Card?
            card_value.get_sprite_index()
        } else {
            CARD_BACK_SPRITE_INDEX // Card back sprite index
        }
    }
}
