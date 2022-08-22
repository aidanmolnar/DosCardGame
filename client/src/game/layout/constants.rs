use super::assets::{CARD_HEIGHT,CARD_WIDTH};

// Constants for displaying game interface
pub const HEIGHT_SCALE: f32 = 2000.; // The fixed logical height of the window

pub const DECK_LOCATION: (f32, f32) = (200.,0.);
pub const DISCARD_LOCATION: (f32, f32) = (-200.,0.);
pub const STAGING_LOCAITON: (f32, f32) = (0., -400.);

pub const MAX_HAND_WIDTH: f32 = 3000.;
pub const MAX_HAND_SPACING: f32 = 80.;
pub const YOUR_HAND_CENTER: (f32, f32) = (0., -1000. + 360. / 2.);

pub const OPPONENT_ARC_WIDTH: f32 = 1500.;
pub const OPPONENT_ARC_HEIGHT: f32 = 800.;
pub const MAX_OPPONENT_HAND_WIDTH: f32 = (MAX_HAND_WIDTH - OPPONENT_ARC_WIDTH) / 2. - 250.;
pub const OPPONENT_ARC_ANGLE: f32 = std::f32::consts::PI * 0.8;

// Mouse over highlighting
pub const HIGHLIGHT_SCALE: f32 = 1.25;
pub const WIDTH_OFFSET: f32 = (CARD_WIDTH * HIGHLIGHT_SCALE + CARD_WIDTH) / 2.;
pub const HIGHLIGHT_Y_OFFSET: f32 = CARD_HEIGHT * (HIGHLIGHT_SCALE - 1.) / 2.;