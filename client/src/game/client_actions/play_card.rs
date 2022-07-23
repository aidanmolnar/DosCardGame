use dos_shared::cards::Card;


use bevy::prelude::*;
//use bevy_mod_picking::*;

#[derive(Component)]
pub struct CardValue (pub Card);

// Run if resource YourTurn exists
// And on event pickable
// pub fn play_card_system (
//     query: Query<&CardValue>,
//     mut events: EventReader<PickingEvent>,
//     mut manager: ResMut<Manager>,
//     mut commands: Commands,
// ) {
//     for event in events.iter() {
//         if let PickingEvent::Clicked(entity) = event {
//             let card_value = query.get(*entity).unwrap();
//             println!("{:?}",card_value.0);

            
//             manager.you_play_card(&mut commands, card_value.0);
//         }
//     }
// }