use crate::multiplayer::MultiplayerState;

use super::{
    super::assets::{AssetState, CardHandles},
    AnimationTable,
};

use bevy::{ecs::system::SystemParam, prelude::*, render::render_resource::Face};
use dos_shared::{table::Location, table_map::TableMap, GameInfo, GameState};
use iyes_loopless::prelude::*;

pub struct CardEffectsPlugin;

impl Plugin for CardEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AssetState::Loaded, setup_materials)
            .add_system(turn_effects_system.run_in_state(GameState::InGame))
            .add_system(update_card_effects_system.run_in_state(GameState::InGame))
            .add_event::<UpdateCardEffectsEvent>();
    }
}

#[derive(Resource)]
pub struct CardDimmingMaterials {
    regular: Handle<StandardMaterial>,
    dimmed: Handle<StandardMaterial>,
}

pub struct UpdateCardEffectsEvent {
    pub card_entity: Entity,
    pub location: Location,
}

#[derive(SystemParam)]
pub struct TurnEffectManager<'w, 's> {
    game_info: Res<'w, GameInfo>,
    mp_state: Res<'w, MultiplayerState>,
    dimming_materials: Res<'w, CardDimmingMaterials>,
    glows: Query<'w, 's, &'static mut Visibility>,
    cards: Query<'w, 's, &'static Children>,
    commands: Commands<'w, 's>,
}

impl TurnEffectManager<'_, '_> {
    pub fn update_effects(&mut self, card_entity: Entity, location: &Location) {
        self.set_dim(card_entity, self.should_dim(location));
        self.set_glow(card_entity, self.should_glow(location));
    }

    fn set_dim(&mut self, card_entity: Entity, dim: bool) {
        //
        if dim {
            self.commands
                .entity(card_entity)
                .insert(self.dimming_materials.dimmed.clone());
        } else {
            self.commands
                .entity(card_entity)
                .insert(self.dimming_materials.regular.clone());
        }
    }

    fn set_glow(&mut self, card_entity: Entity, glow: bool) {
        let card_children = self.cards.get(card_entity).unwrap();
        let glow_entity = card_children.first().unwrap();
        self.glows.get_mut(*glow_entity).unwrap().is_visible = glow;
    }

    fn should_dim(&self, location: &Location) -> bool {
        match location {
            // It is player's hand
            Location::Hand { player_id } if *player_id == self.mp_state.turn_id => {
                // And it's not player's turn
                self.game_info.current_turn() != self.mp_state.turn_id
            }
            // It isn't player's hand
            _ => false,
        }
    }

    fn should_glow(&self, location: &Location) -> bool {
        if let Location::Hand { player_id } = location {
            // If it's a player's hand and it's that players turn
            // And it's not the local player
            (*player_id == self.game_info.current_turn()) && *player_id != self.mp_state.turn_id
            // TODO: make playable cards glow
        } else {
            false
        }
    }
}

fn setup_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    atlases: Res<Assets<TextureAtlas>>,
    card_handles: Res<CardHandles>,
) {
    let atlas = atlases.get(&card_handles.atlas).unwrap();

    let regular = materials.add(StandardMaterial {
        base_color_texture: Some(atlas.texture.clone()),
        cull_mode: Some(Face::Back),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        perceptual_roughness: 0.5,
        reflectance: 0.15,
        //emissive,
        ..Default::default()
    });

    let dimmed = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        base_color_texture: Some(atlas.texture.clone()),
        cull_mode: Some(Face::Back),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        perceptual_roughness: 0.5,
        reflectance: 0.15,
        ..Default::default()
    });

    commands.insert_resource(CardDimmingMaterials { regular, dimmed });
}

fn turn_effects_system(
    mut effect_manager: TurnEffectManager,
    mut last_turn: Local<usize>,
    map: Res<TableMap>,
    tables: Query<&AnimationTable>,
) {
    let turn = effect_manager.game_info.current_turn();

    if turn == *last_turn {
        return;
    }

    // Activate effect for current player
    let table_entity = map.0.get(&Location::Hand { player_id: turn }).unwrap();
    let table = tables.get(*table_entity).unwrap();

    for card_entity in table.iter_entities() {
        if turn == effect_manager.mp_state.turn_id {
            effect_manager.set_dim(*card_entity, false);
        } else {
            effect_manager.set_glow(*card_entity, true);
        }
    }

    // Deactivate effect for last player
    let table_entity = map
        .0
        .get(&Location::Hand {
            player_id: *last_turn,
        })
        .unwrap();
    let table = tables.get(*table_entity).unwrap();

    for card_entity in table.iter_entities() {
        if *last_turn == effect_manager.mp_state.turn_id {
            effect_manager.set_dim(*card_entity, true);
        } else {
            effect_manager.set_glow(*card_entity, false);
        }
    }

    *last_turn = turn;
}

fn update_card_effects_system(
    mut events: EventReader<UpdateCardEffectsEvent>,
    mut effect_manager: TurnEffectManager,
) {
    for event in events.iter() {
        effect_manager.update_effects(event.card_entity, &event.location);
    }
}
