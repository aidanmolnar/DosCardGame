
use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, HoverEvent};

use super::InterfaceManager;

// Only run on picking event
pub fn focus_system (
    mut commands: Commands,
    mut interface_manager: ResMut<InterfaceManager>,
    mut events: EventReader<PickingEvent>,
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
        interface_manager.set_focused_card(&mut commands, option);
    }
}
