use std::f32::consts::FRAC_1_SQRT_2;

use raylib::prelude::*;

pub fn poll_movement_from_keyboard(rl: &RaylibHandle) -> Vector2 {
    let is_w_pressed = rl.is_key_down(KeyboardKey::KEY_W);
    let is_a_pressed = rl.is_key_down(KeyboardKey::KEY_A);
    let is_s_pressed = rl.is_key_down(KeyboardKey::KEY_S);
    let is_d_pressed = rl.is_key_down(KeyboardKey::KEY_D);

    match (is_w_pressed, is_a_pressed, is_s_pressed, is_d_pressed) {
        (true, false, false, false) => Vector2::new(0., -1.),
        (false, true, false, false) => Vector2::new(-1., 0.),
        (false, false, true, false) => Vector2::new(0., 1.),
        (false, false, false, true) => Vector2::new(1., 0.),

        (true, true, false, false) => Vector2::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        (false, true, true, false) => Vector2::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        (false, false, true, true) => Vector2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        (true, false, false, true) => Vector2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),

        _ => Vector2::zero(),
    }
}

pub fn poll_movement(rl: &RaylibHandle) -> Vector2 {
    poll_movement_from_keyboard(&rl)
}
