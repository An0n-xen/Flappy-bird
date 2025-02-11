extern crate piston_window;
extern  crate rand;

mod game;

use piston_window::*;
use crate::game::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Flappy Bird", [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| {
            panic!("Failed to build PistonWindow: {}", e);
        });

    let mut game = Game::new(3);

    while let Some(event) = window.next() {
        if let Some(_args) = event.update_args() {
            game.update();
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::R => game.reset(),
                Key::Up => game.jump(),
                _ => (),
            }
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);

            // Flip coordinate system: origin is bottom left
            let flipped_transform = c.transform
                .trans(0.0, WINDOW_HEIGHT)
                .scale(1.0, -1.0);

            // Draw pipes
            for pipe in &game.pipes {
                let bottom_height = if pipe.gap_y - GAP_SIZE / 2.0 > 0.0 {
                    pipe.gap_y - GAP_SIZE / 2.0
                } else {
                    0.0
                };
                let top_height = if pipe.gap_y + GAP_SIZE / 2.0 < WINDOW_HEIGHT {
                    WINDOW_HEIGHT - (pipe.gap_y + GAP_SIZE / 2.0)
                } else {
                    0.0
                };
                rectangle(
                    [1.0, 0.0, 0.0, 1.0],
                    [pipe.x, 0.0, BLOCK_SIZE, bottom_height],
                    flipped_transform,
                    g,
                );
                rectangle(
                    [1.0, 0.0, 0.0, 1.0],
                    [pipe.x, pipe.gap_y + GAP_SIZE / 2.0, BLOCK_SIZE, top_height],
                    flipped_transform,
                    g,
                );
            }

            // Draw the player's block (bird)
            rectangle(
                [0.0, 0.0, 1.0, 1.0],
                [game.block_x, game.block_y, BLOCK_SIZE, BLOCK_SIZE],
                c.transform,
                g,
            );
        });
    }
}