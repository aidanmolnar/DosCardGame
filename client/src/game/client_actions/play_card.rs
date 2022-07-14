use dos_shared::cards::Card;

use super::InterfaceManager;

use bevy::prelude::*;
use bevy_mod_picking::*;

#[derive(Component)]
pub struct CardValue (pub Card);

// Run if resource YourTurn exists
// And on event pickable
pub fn play_card_system (
    query: Query<&CardValue>,
    mut events: EventReader<PickingEvent>,
    mut interface_manager: ResMut<InterfaceManager>,
    mut commands: Commands,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(entity) = event {
            let card_value = query.get(*entity).unwrap();
            println!("{:?}",card_value.0);

            interface_manager.focused_card = None;
            interface_manager.you_play_card(&mut commands, card_value.0);
        }
    }
}