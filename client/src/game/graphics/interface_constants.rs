
// Constants for displaying game interface

pub const HEIGHT_SCALE: f32 = 2000.; // The fixed logical height of the window

pub const DECK_LOCATION: (f32, f32) = (0.,0.);

pub const MAX_HAND_WIDTH: f32 = 3000.;
pub const MAX_HAND_SPACING: f32 = 80.;
pub const YOUR_HAND_CENTER: (f32, f32) = (0., -1000. + 360. / 2.);

pub const OPPONENT_ARC_WIDTH: f32 = 1500.;
pub const OPPONENT_ARC_HEIGHT: f32 = 600.;
pub const MAX_OPPONENT_HAND_WIDTH: f32 = (MAX_HAND_WIDTH - OPPONENT_ARC_WIDTH) / 2. - 250.;
pub const OPPONENT_ARC_ANGLE: f32 = std::f32::consts::PI * 0.8;