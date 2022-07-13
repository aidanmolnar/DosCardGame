
use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, HoverEvent};

#[derive(Default)]
pub struct FocusedCard (pub Option<Entity>);

use super::game_manager::AnimateCards;

// Only run on picking event
pub fn card_focus_system (
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mut focused_card: ResMut<FocusedCard>,
) {
    let mut option = None;
    let mut update_event = false;

    for event in events.iter() {
        if let PickingEvent::Hover(hover_event) = event {
            update_event = true;

            match hover_event {
                HoverEvent::JustEntered(entity) => {option = Some(*entity)}
                HoverEvent::JustLeft(_) => {}
            }
        }
    }
    
    if update_event {
        commands.spawn().insert(AnimateCards::YourHand);
        focused_card.0 = option;
    }
}
