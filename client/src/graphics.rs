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



pub fn add_card(
    mut commands: Commands,
    atlas: Res<CardTetxureAtlas>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let card = Card::Basic{color: dos_shared::cards::Color::Green, value: 9};

    let texture_atlas_handle = texture_atlases.get_handle(&atlas.atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite { index: get_index(card), ..default() },
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        });
}

fn get_index(card: Card) -> usize {
    match card {
        Card::Basic { color, value } => {get_color_offset(color) + value as usize},
        Card::Skip {color}               => {get_color_offset(color) + 11},
        Card::Reverse {color}            => {get_color_offset(color) + 12},
        Card::DrawTwo {color}            => {get_color_offset(color) + 13},
        Card::Wild {}                           => {4*13+1},
        Card::DrawFour {}                       => {4*13+2},
    }
}

fn get_color_offset(color: dos_shared::cards::Color) -> usize {
    match color {
        dos_shared::cards::Color::Red    => {   0}
        dos_shared::cards::Color::Yellow => {  13}
        dos_shared::cards::Color::Green  => {2*13}
        dos_shared::cards::Color::Blue   => {3*13}
    }
}

// TODO
pub fn add_camera() {

}


pub fn show_cards_test(
    mut commands: Commands,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    atlas: Res<CardTetxureAtlas>,
) {

    
    
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::FixedVertical;
    camera.orthographic_projection.scale = 1000.0;

    commands.spawn_bundle(camera);


    add_card(commands, atlas, texture_atlases)

    // commands.spawn_bundle(SpriteBundle {
    //     texture: asset_server.load("cards_test.png"),
    //     ..default()
    // });
}