
use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, HoverEvent};

use super::super::super::game_manager::GameManager;

#[derive(Component)]
pub struct MouseOffset {
    pub offset: Vec3,
    pub scale: f32,
}

// Only run on picking event
pub fn focus_system (
    mut commands: Commands,
    mut game_manager: ResMut<GameManager>,
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
        game_manager.tracker.focused_card = option;
        game_manager.tracker.update_your_focus(&mut commands);
    }
}
