// src/main.rs

mod state;
mod input; 

use macroquad::prelude::*;
use crate::state::player::Player;

#[macroquad::main("ArrowIO")]
async fn main() {
    let mut player_pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
    
    let mut player = Player::new(player_pos);


    loop {
        clear_background(BLACK);
        let input = input::get_input(&player.pos);
        
        player.update(&input);
        
        player.draw();
        next_frame().await;
    }
}
