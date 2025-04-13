// src/state/player.rs

use macroquad::prelude::*;
use crate::input::PlayerInput;

pub enum CollisionResult {
    BothDie,
    SelfDies,
    OtherDies,
    None,
}

pub struct Player {
    pub pos: Vec2,
    pub speed: f32,
    pub color: Color,
    pub alive: bool,
    pub direction: Vec2,
    pub dash_target: Option<Vec2>,
    pub last_move_time: f64
}

impl Player {
    pub fn new(pos: Vec2, color: Color) -> Self {
        Self {
            pos, 
            speed: 90.0,
            color,
            last_move_time: -100.0,
            direction: vec2(0.0, 0.0),
            dash_target: None,
            alive: true
        }
    }
    


    /// Returns `true` if player moved this frame
    pub fn update(&mut self, input: &PlayerInput) -> bool {
        let now = get_time();

        // Start dash
        if input.moving && self.dash_target.is_none() && now - self.last_move_time >= 1.0 {
            self.dash_target = Some(self.pos + input.direction * self.speed);
            self.direction = input.direction;
            self.last_move_time = now;
        }

        // Continue dash
        if let Some(target) = self.dash_target {
            let dir = (target - self.pos).normalize_or_zero();
            let step = dir * 10.0;

            self.pos += step;

            if self.pos.distance(target) < 5.0 {
                self.pos = target;
                self.dash_target = None;
            }

            return true; // ✅ player moved this frame
        }

        false
    }


    
    pub fn draw(&self) {
        let mouse = vec2(mouse_position().0, mouse_position().1);
        let dir = (mouse - self.pos).normalize_or_zero();

        let perp = vec2(-dir.y, dir.x); // perpendicular vector

        let tip     = self.pos + dir * 20.0;
        let left    = self.pos - dir * 10.0 + perp * 10.0;
        let right   = self.pos - dir * 10.0 - perp * 10.0;

        draw_triangle(tip, left, right, self.color);
    }

    pub fn tip(&self) -> Vec2 {
        let mouse = vec2(mouse_position().0, mouse_position().1);
        let dir = (mouse - self.pos).normalize_or_zero();
        self.pos + dir * 20.0
    }


    pub fn collides_with(&self, other: &Player) -> CollisionResult {
        let tip_self = self.tip();
        let tip_other = other.tip();

        let tip_to_tip_dist = tip_self.distance(tip_other);
        let self_tip_to_other_center = tip_self.distance(other.pos);
        let other_tip_to_self_center = tip_other.distance(self.pos);

        let collision_radius = 20.0;

        let head_on = tip_to_tip_dist <= collision_radius;
        let self_hits_other = self_tip_to_other_center <= collision_radius;
        let other_hits_self = other_tip_to_self_center <= collision_radius;

        match (head_on, self_hits_other, other_hits_self) {
            (true, _, _) => CollisionResult::BothDie,
            (_, true, false) => CollisionResult::OtherDies,
            (_, false, true) => CollisionResult::SelfDies,
            _ => CollisionResult::None,
        }
    }    
}
