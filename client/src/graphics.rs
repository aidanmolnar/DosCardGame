use dos_shared::cards::*;
use super::lobby_network::MultiplayerState;
use super::game_network::ResourceReference;

use bevy::prelude::*;
use bevy::ecs::event::Events;
use bevy::render::camera::ScalingMode;

// Asset path to card sprites
const CARDS_PATH: &str = "UNO_cards.png";

const CARD_BACK_SPRITE_INDEX: usize = 4*13+2;
const DECK_LOCATION: (f32, f32) = (0.,0.);

// Constants for displaying hands of cards
const MAX_HAND_WIDTH: f32 = 3000.;
const MAX_HAND_SPACING: f32 = 80.;
const YOUR_HAND_CENTER: (f32, f32) = (0., -700.);

const OPPONENT_ARC_WIDTH: f32 = 1500.;
const OPPONENT_ARC_HEIGHT: f32 = 600.;
const MAX_OPPONENT_HAND_WIDTH: f32 = (MAX_HAND_WIDTH - OPPONENT_ARC_WIDTH) / 2. - 250.;
const OPPONENT_ARC_ANGLE: f32 = std::f32::consts::PI * 0.8;



pub struct CardTetxureAtlas {
    atlas: Handle<TextureAtlas>,
}

pub fn load_assets (
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

pub fn add_camera(
    mut commands: Commands
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::FixedVertical;
    camera.orthographic_projection.scale = 1024.;

    commands.spawn_bundle(camera);
}


#[derive(Component)]
pub struct Target {
    start: Vec3,
    end: Vec3,
    timer: Timer,
}

pub fn deal_card (
    owner_id: u8,
    card_value: Option<Card>,
    r: &mut ResourceReference,
) {
    // Get sprite info
    let index = if let Some(card_value) = card_value {
        get_index(&card_value)
    } else {
        CARD_BACK_SPRITE_INDEX // Card back sprite index
    };
    let texture_atlas_handle = r.texture_atlases.get_handle(&r.card_atlas.atlas);
    
    let entity = spawn_card_entity(
        &mut r.commands,
        index,
        texture_atlas_handle,
    );
    
    // Add the card to the card tracker (TODO: break into another function)
    r.card_tracker.add_card(
        card_value,
        entity,
        owner_id,
        r.mp_state.turn_id,
    );

    // Sends event to update card target locations
    r.events.send(CardChanged {owner_id})
}

fn spawn_card_entity(
    commands: &mut Commands,
    sprite_index: usize,
    texture_atlas_handle: Handle<TextureAtlas>,
) -> Entity {
    let translation = Vec3::new(DECK_LOCATION.0, DECK_LOCATION.1, 0.);

    commands
    .spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite { index: sprite_index, ..default() },
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(translation).with_scale(Vec3::splat(1.0)),
        ..default()
    }).insert(
        Target {
        start: translation,
        end: translation,
        timer: Timer::from_seconds(0.01,false),
    }).id() 
}

// System for animating cards to their target locations
pub fn move_targets (
    mut query: Query<(&mut Target, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut target, mut transform) in query.iter_mut() {
        target.timer.tick(time.delta());
        transform.translation = target.start + (target.end - target.start) * target.timer.percent();
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
pub struct CardTracker {
    your_cards: Vec<Card>,
    map: Vec<Vec<Entity>>,
}

impl CardTracker {
    fn add_card(
        &mut self,
        card: Option<Card>,
        card_entity: Entity,
        card_owner_id: u8,
        your_id: u8,
    ) {
        // Handles case where the card owner is the local player
        if card_owner_id == your_id {
            let hand_position = self.your_cards.binary_search_by(|x| x.cmp(&card.unwrap())).unwrap_or_else(|x| x);
            self.your_cards.insert(hand_position, card.unwrap());
    
            // Inserts into card tracker, maintaining sorted order by card value
            if let Some(vec) = self.map.get_mut(your_id as usize) {
                vec.insert(hand_position, card_entity);
            } else {
                self.map.push(vec![card_entity]);
            }
        // Handles case where the card owner is an opponent
        } else if let Some(vec) = self.map.get_mut(card_owner_id as usize) {
            vec.push(card_entity);
        } else {
            self.map.push(vec![card_entity]);
        }
        // TODO: Consider teammates?
    }
}


pub struct HandLocations{
    pub centers: Vec<(f32,f32)>,
}

// TODO: split this up
pub fn setup_graphics(
    mut commands: Commands,
    mp_state: Res<MultiplayerState>,
) {
    commands.init_resource::<CardTracker>();

    let num_players = mp_state.player_names.len() as u8;
    let num_other_players = num_players - 1;

    let mut centers = Vec::new();

    for owner_id in 0..num_players {

        if owner_id == mp_state.turn_id{
            centers.push(YOUR_HAND_CENTER);
        } else {
            // Adjust other ids so your hand is skipped
            let local_id = if owner_id > mp_state.turn_id{
                (owner_id-1) % num_other_players
            } else {
                owner_id % num_other_players
            };
        
            // Arrange centers of opponents hands in an arc
            let (x,y) = arange_arc(
                num_other_players as usize, 
                local_id as usize,
                OPPONENT_ARC_ANGLE);
            let center_x = OPPONENT_ARC_WIDTH*x;
            let center_y = OPPONENT_ARC_HEIGHT*y;

            centers.push( (center_x,center_y));
        }
        
    }

    commands.insert_resource(HandLocations{centers})
    
}



pub struct CardChanged {owner_id: u8}

pub fn set_card_targets (
    mut query: Query<(&mut Target, &Transform)>,
    other_locations: Res<HandLocations>,
    card_tracker: Res<CardTracker>,
    mut events: ResMut<Events<CardChanged>>,
    mp_state: Res<MultiplayerState>,
) {
    for event in events.drain() {

        let hand = card_tracker.map.get(event.owner_id as usize).unwrap();
        let (center_x, center_y) = other_locations.centers.get(event.owner_id as usize).unwrap();

        let max_width = if event.owner_id == mp_state.turn_id {
            f32::min(MAX_HAND_WIDTH, hand.len() as f32 * MAX_HAND_SPACING)
        } else {
            f32::min(MAX_OPPONENT_HAND_WIDTH, hand.len() as f32 * MAX_HAND_SPACING)
        };

        for (hand_position, entity) in hand.iter().enumerate() {
            let (mut target, transform) = query.get_mut(*entity).unwrap();
            let pos = max_width * arange_1d(hand.len(), hand_position); 
            let new_dest = Vec3::new(*center_x + pos, *center_y, (hand_position as f32) / 10.);

            target.start = transform.translation;
            target.end = new_dest;
            target.timer = Timer::from_seconds(2., false);
        }
    }
}