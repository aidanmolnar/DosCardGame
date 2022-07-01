use dos_shared::cards::*;
use super::lobby_network::MultiplayerState;

use bevy::prelude::*;
use bevy::ecs::event::Events;
use bevy::render::camera::ScalingMode;

use std::collections::HashMap;

pub const CARDS_PATH: &str = "UNO_cards.png";

pub struct CardTetxureAtlas {
    atlas: Handle<TextureAtlas>,
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands
) {
    let texture_handle = asset_server.load(CARDS_PATH);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(240.0, 360.0), 13, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(CardTetxureAtlas{atlas: texture_atlas_handle})
}

fn get_index(card: &Card) -> usize {
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


// TODO
pub fn add_camera(
    mut commands: Commands
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::FixedVertical;
    camera.orthographic_projection.scale = 1024.;

    commands.spawn_bundle(camera);
}

#[derive(Component)]
pub struct YourCard {
    card_value: Card,
}

#[derive(Component)]
pub struct OtherCard {
    owner_id: u8,
    hand_position: u8,
}

#[derive(Component)]
pub struct Target {
    start: Vec3,
    end: Vec3,
    timer: Timer,
}

pub fn add_your_card (
    card_value: Card,
    translation: Vec3,
    commands: &mut Commands,
    atlas: &Res<CardTetxureAtlas>,
    texture_atlases: &Res<Assets<TextureAtlas>>,
) {
    let texture_atlas_handle = texture_atlases.get_handle(&atlas.atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite { index: get_index(&card_value), ..default() },
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(translation).with_scale(Vec3::splat(1.0)),
            ..default()
        }).insert(
            YourCard {
            card_value,
        }).insert(
            Target {
            start: translation,
            end: translation,
            timer: Timer::from_seconds(0.01,false),
        }); 
}

// TODO: make this a lot less ugly (fewer args)
pub fn add_other_card (
    owner_id: u8,
    hand_position: u8,
    translation: Vec3,
    commands: &mut Commands,
    atlas: &Res<CardTetxureAtlas>,
    texture_atlases: &Res<Assets<TextureAtlas>>,
    card_tracker: &mut ResMut<OpponentCardTracker>,
    events: &mut EventWriter<OpponentCardChanged>,
) {
    let texture_atlas_handle = texture_atlases.get_handle(&atlas.atlas);
    let entity = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite { index: 4*13+2, ..default() },
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(translation).with_scale(Vec3::splat(1.0)),
            ..default()
        }).insert(
            OtherCard {
            owner_id,
            hand_position,
        }).insert(
            Target {
            start: translation,
            end: translation,
            timer: Timer::from_seconds(0.01,false),
        }).id(); 
    
    // Inserts into card tracker, maintaining sorted order
    if let Some(vec) = card_tracker.map.get_mut(&owner_id) {
        let index = vec.binary_search_by_key(&hand_position, |x| x.0).unwrap_or_else(|x| x);
        vec.insert(index, (hand_position, entity));
    } else {
        card_tracker.map.insert(owner_id, vec![(hand_position, entity)]);
    }

    // Sends event to update card target locations
    events.send(OpponentCardChanged { id: owner_id })
}

pub const MAX_HAND_WIDTH: f32 = 3000.;
pub const MAX_HAND_SPACING: f32 = 80.;

pub fn move_targets (
    mut query: Query<(&mut Target, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut target, mut transform) in query.iter_mut() {
        target.timer.tick(time.delta());
        transform.translation = target.start + (target.end - target.start) * target.timer.percent();
    }
}  


// TODO: Don't do this every frame, only when a card is played, drawn, or disturbed by mouse
pub fn set_targets_your_cards (
    mut query: Query<(&mut Target, &Transform, &YourCard)>,
) {

    // TODO: Don't do this every frame.  Only when card is played or drawn
    // Sort entities by the card type
    let mut entities  = query.iter_mut().collect::<Vec<_>>(); // TODO: filter out locked entities
    entities.sort_by_key(|e| e.2.card_value);

    // Clamps hand width while also keeping cards clustered
    let len = entities.len();
    let max = f32::min(MAX_HAND_WIDTH, len as f32 * MAX_HAND_SPACING);

    for (i, (target, transform, _)) in entities.iter_mut().enumerate() {

        // TODO: this might be skippable? can wait for current animation to end?
        // Calculate the intended destination of the card
        let pos = max * arange_1d(len, i); 

        let new_dest = Vec3::new(pos,-700.,(i as f32) / 10. );

        if new_dest != target.end {
            println!("recalculating {}", new_dest);
            target.start = transform.translation;
            target.end = new_dest;
            target.timer = Timer::from_seconds(2., false);
        }
        
    }

}

// Returns a value between -0.5 and 0.5 based on position in array
fn arange_1d(len: usize, i: usize) -> f32 {
    if len > 1 {
        (i as f32 / (len-1) as f32) - 0.5
    } else {
         0.
    }
}

// Returns an (x,y) pair on unit circle between -angle / 2 and angle / 2 based on position in array
fn arange_arc(len: usize, i: usize, angle: f32) -> (f32, f32) {
    f32::sin_cos(angle * arange_1d(len, i))
}

#[derive(Default)]
pub struct OpponentCardTracker {
    map: HashMap<u8, Vec<(u8, Entity)>>,
}

const OPPONENT_ARC_WIDTH: f32 = 1500.;
const OPPONENT_ARC_HEIGHT: f32 = 600.;
const MAX_OPPONENT_HAND_WIDTH: f32 = (MAX_HAND_WIDTH - OPPONENT_ARC_WIDTH) / 2. - 250.;
const OPPONENT_ARC_ANGLE: f32 = std::f32::consts::PI * 0.8;

pub struct OpponentHandLocations{
    pub centers: HashMap<u8, (f32,f32)>,
}

// TODO: split this up
pub fn setup_graphics(
    mut commands: Commands,
    mp_state: Res<MultiplayerState>,
) {
    commands.init_resource::<OpponentCardTracker>();

    let num_players = mp_state.player_names.len();
    let num_other_players = num_players - 1;

    let mut centers = HashMap::new();

    for owner_id in 0..num_players {

        // Adjust other ids so your hand is skipped
        let local_id = if owner_id > mp_state.turn_id as usize {
            (owner_id-1) % num_other_players
        } else {
            owner_id % num_other_players
        };
    
        // Arrange centers of opponents hands in an arc
        let (x,y) = arange_arc(
            num_other_players, 
            local_id as usize,
            OPPONENT_ARC_ANGLE);
        let center_x = OPPONENT_ARC_WIDTH*x;
        let center_y = OPPONENT_ARC_HEIGHT*y;

        centers.insert(owner_id as u8, (center_x,center_y));
    }

    commands.insert_resource(OpponentHandLocations{centers})
    
}



pub struct OpponentCardChanged {id: u8}

pub fn set_targets_other_cards (
    mut query: Query<(&mut Target, &Transform, &OtherCard)>,
    other_locations: Res<OpponentHandLocations>,
    card_tracker: Res<OpponentCardTracker>,
    mut events: ResMut<Events<OpponentCardChanged>>,
) {
    for event in events.drain() {
        let hand = card_tracker.map.get(&event.id).unwrap();
        let (center_x, center_y) = other_locations.centers.get(&event.id).unwrap();
        let max_width = f32::min(MAX_OPPONENT_HAND_WIDTH, hand.len() as f32 * MAX_HAND_SPACING);

        for (hand_position, entity) in hand.iter() {
            let (mut target, transform, _) = query.get_mut(*entity).unwrap();
            let pos = max_width * arange_1d(hand.len(), *hand_position as usize); 
            let new_dest = Vec3::new(*center_x + pos, *center_y, (*hand_position as f32) / 10.);

            target.start = transform.translation;
            target.end = new_dest;
            target.timer = Timer::from_seconds(2., false);
        }
    }

}