use super::assets::{CARD_HEIGHT, CARD_WIDTH};

/// Constants for displaying game interface
pub const DECK_LOCATION: (f32, f32) = (200.,0.);
pub const DISCARD_LOCATION: (f32, f32) = (-200.,0.);
pub const STAGING_LOCATION: (f32, f32) = (0., -400.);

pub const MAX_HAND_WIDTH: f32 = 3000.; 
pub const MAX_HAND_SPACING: f32 = 80.; // Limit on how far apart cards can be in hand. Looks moure natural when they are always overlapping a bit.
pub const YOUR_HAND_CENTER: (f32, f32) = (0., -1000. + 360. / 2.);

pub const OPPONENT_ARC_WIDTH: f32 = 1500.;
pub const OPPONENT_ARC_HEIGHT: f32 = 800.;
pub const MAX_OPPONENT_HAND_WIDTH: f32 = (MAX_HAND_WIDTH - OPPONENT_ARC_WIDTH) / 2. - 250.; // Keeps opponents hands on screen at default aspect ratio
pub const OPPONENT_ARC_ANGLE: f32 = std::f32::consts::PI * 0.8;

/// Mouse over highlighting constantcs
pub const HIGHLIGHT_SCALE: f32 = 1.25;
pub const WIDTH_OFFSET: f32 = (CARD_WIDTH * HIGHLIGHT_SCALE + CARD_WIDTH) / 2.; // Distance to move non-hovered cards horizontally so that they don'y overlap with hovered card
pub const HIGHLIGHT_Y_OFFSET: f32 = CARD_HEIGHT * (HIGHLIGHT_SCALE - 1.) / 2.; // Distance to move enlarged cards up to keep the bottom alligned with other cards