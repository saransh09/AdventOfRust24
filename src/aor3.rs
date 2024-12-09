// We need to find the nice and naughty kids for santa

// Each good deed is worth 1 point and each bad deed is worth 2 points
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    (good_deeds as f32 * GOOD_WEIGHT)
        / (good_deeds as f32 * GOOD_WEIGHT + bad_deeds as f32 * BAD_WEIGHT) >= 0.75
}
