
use super::constants::*;

// TODO: clean this up?, make it based on width?  How does this relate to table arranger
pub fn your_max_hand_width(hand_size: usize) -> f32 {
    f32::min(MAX_HAND_WIDTH, hand_size as f32 * MAX_HAND_SPACING)
}

// pub fn opponent_max_hand_width(hand_size: usize) -> f32 {
//     f32::min(MAX_OPPONENT_HAND_WIDTH, hand_size as f32 * MAX_HAND_SPACING)
// }

// Returns a value between -0.5 and 0.5 based on position in array
pub fn arange_1d(len: usize, i: usize) -> f32 {
    if len > 1 {
        (i as f32 / (len-1) as f32) - 0.5
    } else {
         0.
    }
}

// Returns an (x,y) pair on unit circle between -angle / 2 and angle / 2 based on position in array
pub fn arange_arc(len: usize, i: usize, angle: f32) -> (f32, f32) {
    f32::sin_cos(angle * arange_1d(len, i))
}

// TODO: what is this?
pub fn horizontal_offset(hand_size: usize) -> f32 {
    if hand_size == 0 {
        0.
    } else {
        let hand_spacing = your_max_hand_width(hand_size) / (hand_size - 1) as f32; 
        WIDTH_OFFSET - hand_spacing
    }
}