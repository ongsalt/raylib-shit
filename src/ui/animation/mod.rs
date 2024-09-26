use std::time::Instant;

pub mod spring;

pub trait ValueAnimator {
    fn value(&self, time: Instant) -> f32;
    fn set_target(&mut self, target: f32, set_time: Instant);
}