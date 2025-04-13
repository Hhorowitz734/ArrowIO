// src/input.rs
use macroquad::prelude::*;

pub struct PlayerInput {
    pub direction: Vec2,
    pub moving: bool
}

pub fn get_input(player_pos: &Vec2) -> PlayerInput {
    let mouse = mouse_position();
    let mouse_pos = vec2(mouse.0, mouse.1);

    let raw_dir = mouse_pos - *player_pos;
    let direction = raw_dir.normalize_or_zero();

    PlayerInput {
        direction,
        moving: is_key_pressed(KeyCode::Space),
    }
}
