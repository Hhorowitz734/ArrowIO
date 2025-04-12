// src/state/player.rs

use macroquad::prelude::*;
use crate::input::PlayerInput;

pub struct Player {
    pub pos: Vec2,
    pub speed: f32,
    last_move_time: f64
}

impl Player {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos, 
            speed: 90.0,
            last_move_time: -100.0
        }
    }
    
    pub fn update(&mut self, input: &PlayerInput) {
        let now = get_time();

        if input.moving && now - self.last_move_time > 2.0 {
            self.pos += input.direction * self.speed;
            self.last_move_time = now;
        }
    }


    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 20.0, RED);
    }
}
