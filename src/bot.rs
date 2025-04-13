
// src/bot.rs

use macroquad::prelude::*;
use crate::state::player::Player;
use crate::input;
/// Spawns a list of AI-controlled red bots at fixed positions.
pub fn generate_bots() -> Vec<Player> {
    let bot_positions = [
        vec2(100.0, 100.0),
        vec2(300.0, 200.0),
        vec2(500.0, 400.0),
        vec2(500.0, 300.0),
        vec2(500.0, 200.0),
        vec2(500.0, 100.0),
        vec2(333.0, 100.0),
        vec2(220.0, 100.0),
        vec2(500.0, 100.0),
    ];

    bot_positions
        .iter()
        .map(|&pos| Player::new(pos, RED))
        .collect()
}

/// Draws all alive bots to the screen.
pub fn draw_bots(bots: &[Player]) {
    for bot in bots {
        if bot.alive {
            bot.draw();
        }
    }
}

/// Updates bot movement at random intervals with random directions.
pub fn update_bots(bots: &mut [Player]) {
    for bot in bots.iter_mut() {
        if !bot.alive {
            continue;
        }

        let now = get_time();

        if now - bot.last_move_time >= rand::gen_range(1.5, 3.0) {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let dir = vec2(angle.cos(), angle.sin());

            let fake_input = crate::input::PlayerInput {
                direction: dir,
                moving: true,
            };

            bot.update(&fake_input);
        }
    }
}

