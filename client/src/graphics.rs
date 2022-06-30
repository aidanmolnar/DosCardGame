use dos_shared::cards::*;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

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
pub struct PlayingCard {
    card: Card,
}

#[derive(Component)]
pub struct Target {
    start: Vec3,
    end: Vec3,
    timer: Timer,
}

pub fn add_card(
    card: &Card,
    translation: Vec3,
    commands: &mut Commands,
    atlas: &Res<CardTetxureAtlas>,
    texture_atlases: &Res<Assets<TextureAtlas>>,
) {

    let texture_atlas_handle = texture_atlases.get_handle(&atlas.atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite { index: get_index(card), ..default() },
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(translation).with_scale(Vec3::splat(1.0)),
            ..default()
        }).insert(
            PlayingCard {
            card: *card,
        }).insert(
            Target {
            start: translation,
            end: translation,
            timer: Timer::from_seconds(0.01,false),
        }); 
}

pub const MAX_HAND_WIDTH: f32 = 3000.;
pub const MAX_HAND_SPACING: f32 = 120.;

pub fn move_targets (
    mut query: Query<(&mut Target, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut target, mut transform) in query.iter_mut() {
        target.timer.tick(time.delta());
        transform.translation = target.start + (target.end - target.start) * target.timer.percent();
    }
}  

pub fn set_targets (
    mut query: Query<(&mut Target, &Transform, &PlayingCard)>,
) {

    // Sort entities by the card type
    let mut entities  = query.iter_mut().collect::<Vec<_>>();
    entities.sort_by_key(|e| e.2.card);

    // Clamps hand width while also keeping cards clustered
    let len = entities.len();
    let max = f32::min(MAX_HAND_WIDTH, len as f32 * MAX_HAND_SPACING);

    for (i, (target, transform, player)) in entities.iter_mut().enumerate() {

        // TODO: this might be skippable? can wait for current animation to end?
        // Calculate the intended destination of the card
        let pos: f32 = max * ((i as f32 / (len-1) as f32) - 0.5);
        let new_dest = Vec3::new(pos,-700.,0.);

        if new_dest != target.end {
            println!("recalculating {}", new_dest);
            target.start = transform.translation;
            target.end = new_dest;
            target.timer = Timer::from_seconds(2., false);
        }
        
    }

}