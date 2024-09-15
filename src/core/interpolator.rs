pub type Interpolator = fn(fraction: f32) -> f32;

pub const CUBIC_EASE_OUT: Interpolator = |fraction| 1. - (1. - fraction).powi(3);
