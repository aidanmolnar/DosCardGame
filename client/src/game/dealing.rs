use dos_shared::{cards::*, NUM_STARTING_CARDS};

use super::MultiplayerState;
use super::card_tracker::CardTracker;

// TODO: move functions that need these into graphics
use super::graphics::card_indexing::{get_index,CARD_BACK_SPRITE_INDEX};
use super::graphics::animations::{LinearAnimation, HandUpdated};
use super::graphics::interface_constants::*;
use super::graphics::assets::CardHandles;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::ecs::system::SystemParam;
use bevy_mod_picking::PickableBundle;

// TODO: maybe move deeper down module tree
pub fn deal_card (
    owner_id: u8,
    card_value: Option<Card>,
    r: &mut DealCardResources,
) {
    // Get sprite info
    let index = if let Some(card_value) = card_value {
        get_index(&card_value)
    } else {
        CARD_BACK_SPRITE_INDEX // Card back sprite index
    };
    let texture_atlas_handle = r.texture_atlases.get_handle(&r.card_handles.atlas);
    
    let entity = spawn_card_entity(
        index,
        texture_atlas_handle,
        r,
        owner_id == r.mp_state.turn_id,
    );
    
    // Add the card to the card tracker (TODO: break into another function)
    r.card_tracker.add_card(
        card_value,
        entity,
        owner_id,
        r.mp_state.turn_id,
    );

    // Sends event to update card target locations
    r.events.send(HandUpdated{owner_id})
}

// TODO: move deeper down module tree
fn spawn_card_entity(
    //commands: &mut Commands,
    sprite_index: usize,
    texture_atlas_handle: Handle<TextureAtlas>,
    r: &mut DealCardResources,
    pickable: bool,
) -> Entity {
    let translation = Vec3::new(DECK_LOCATION.0, DECK_LOCATION.1, 0.);

    let mut entity_commands = r.commands.spawn();
    entity_commands.insert_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite { index: sprite_index, ..default() },
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(translation).with_scale(Vec3::splat(1.0)),
        ..default()
    });
    
    
    entity_commands.insert( LinearAnimation {
        start: translation,
        end: translation,
        timer: Timer::from_seconds(0.01,false),
    });
    
    if pickable {
        println!("added pickable");
        entity_commands
            .insert_bundle(MaterialMesh2dBundle {
                mesh: r.meshes.add(Mesh::from(shape::Quad::new(Vec2::new(240.,360.)))).into(),
                material: r.materials.add(ColorMaterial::from(Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0. })),
                ..default()
            });

        entity_commands.insert_bundle(PickableBundle::default());
    }
    
    entity_commands.id() 
}

#[derive(Component)]
pub struct DelayedDealtCard {
    pub timer: Timer,
    pub owner_id: u8,
    pub card_value: Option<Card>,
}

pub fn delayed_dealing_system (
    mut query: Query<(Entity, &mut DelayedDealtCard)>,
    time: Res<Time>,
    mut commands: Commands,
    mut deal_card_resources: DealCardResources,
) {
    for (entity, mut delayed_card) in query.iter_mut() {
        delayed_card.timer.tick(time.delta());

        if delayed_card.timer.finished() {
            deal_card(
                delayed_card.owner_id,
                delayed_card.card_value,
                &mut deal_card_resources
            );
            commands.entity(entity).remove::<DelayedDealtCard>();
        }

        
    }
}

// Move to graphics?
pub fn deal_out_cards(
    your_cards: Vec<Card>, 
    mut card_counts: Vec<u8>,
    mut commands: Commands,
    mp_state: ResMut<MultiplayerState>,
) {

    let delay_delta = 0.25;
    let mut delay_total = 0.0;

    // Deal out the hands from the deck
    // This is probably more complicated than it needs to be, can make assumptions about how server deals out cards.  Remove card counts from message?
    // TODO: Simplify
    for j in 0..NUM_STARTING_CARDS {
        for (card_owner_id,count) in card_counts.iter_mut().enumerate() {
            if *count > 0 {
                *count -= 1;

                let card_value = if card_owner_id == mp_state.turn_id as usize {
                    Some(*your_cards.get(j as usize).unwrap())
                } else {
                    None
                };

                commands.spawn().insert(DelayedDealtCard {
                    timer: Timer::from_seconds(delay_total, false),
                    owner_id: card_owner_id as u8,
                    card_value,
                });

                delay_total += delay_delta;
                
            } else {
                break;
            }
        }
    }
}

// Bundle of resources to make passing information to functions cleaner
// TODO: rename/reconsider
#[derive(SystemParam)] 
pub struct DealCardResources<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub mp_state: ResMut<'w, MultiplayerState>, 
    pub card_tracker: ResMut<'w, CardTracker>,
    pub events: EventWriter<'w,'s,  HandUpdated>,
    pub texture_atlases: Res<'w, Assets<TextureAtlas>>,
    pub card_handles: Res<'w, CardHandles>,
    pub meshes : ResMut<'w, Assets<Mesh>>,
    pub materials : ResMut<'w, Assets<ColorMaterial>>,
}