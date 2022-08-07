use dos_shared::cards::Card;
use dos_shared::table::{CardReference, Location};
use dos_shared::valid_move;

use crate::game::table::FocusedCard;
use crate::game::table::CardTransferer;
use crate::multiplayer::MultiplayerState;

use bevy::prelude::*;
use bevy_mod_picking::*;

#[derive(Component)]
pub struct CardValue (pub Card);

// Run if resource YourTurn exists
// And on event pickable
pub fn play_card_system (
    mp_state: Res<MultiplayerState>,
    mut events: EventReader<PickingEvent>,
    focused_card: Res<FocusedCard>,
    mut card_transferer: CardTransferer,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(_) = event {

            if let Some((location, table_index_data)) = &focused_card.0 {


                if let Some(card_value) = table_index_data.get_card_value() {
                    if let Some(discard_value) = card_transferer.peek_discard() {
                        println!("{:?}", discard_value);

                        if valid_move(card_value, discard_value) && 
                            *location == (Location::Hand{player_id: mp_state.turn_id})
                        {

                            card_transferer.transfer(
                                table_index_data.to_card_reference(*location), 
                                CardReference{location: Location::DiscardPile, index: None}, 
                                Some(card_value),
                            )
                        }
                    }
                }
                
            }
        }
    }
}