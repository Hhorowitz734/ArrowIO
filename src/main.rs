
// src/main.rs

mod state;
mod input;
mod bot;

use macroquad::prelude::*;
use crate::state::player::{Player, CollisionResult};
use bot::{generate_bots, update_bots, draw_bots};

#[macroquad::main("ArrowIO")]
async fn main() {
    // Initialize the player at the center of the screen
    let mut player = Player::new(
        vec2(screen_width() / 2.0, screen_height() / 2.0),
        BLUE,
    );

    // Generate red AI-controlled bots
    let mut bots = generate_bots();

    loop {
        clear_background(BLACK);

        // Read input for player: direction and left-click action
        let input = input::get_input(&player.pos);

        // Update bots' logic and position
        update_bots(&mut bots);

        // Update player movement; returns true if player moved this frame
        let moved = player.update(&input);

        // Run collision checks only if player moved and is still alive
        if moved && player.alive {
            for bot in bots.iter_mut() {
                if !bot.alive {
                    continue;
                }

                match player.collides_with(bot) {
                    CollisionResult::BothDie => {
                        player.alive = false;
                        bot.alive = false;
                    }
                    CollisionResult::SelfDies => {
                        player.alive = false;
                    }
                    CollisionResult::OtherDies => {
                        bot.alive = false;
                    }
                    CollisionResult::None => {}
                }
            }
        }

        // Draw entities
        if player.alive {
            player.draw();
        }

        draw_bots(&bots);

        next_frame().await;
    }
}

