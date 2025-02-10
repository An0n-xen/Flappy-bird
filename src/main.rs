extern crate piston_window;
extern crate rand;

use piston_window::*;
use piston_window::types::Color;
use rand::Rng;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const BLOCK_SIZE: f64 = 25.0;
const STEP: f64 = 10.0;
const GAP_SIZE: f64 = 90.0;
const WINDOW_HEIGHT: f64 = 360.0;
const WINDOW_WIDTH: f64 = 640.0;
const PIPE_SPEED: f64 = 0.6;

const GRAVITY: f64 = 0.2;
const JUMP_IMPULSE: f64 = -5.0;

struct Pipe {
    x: f64,
    gap_y: f64,
}

// returns true if the two rectangles intersect
fn rects_intersect(ax: f64, ay: f64, aw: f64, ah: f64, bx: f64, by: f64, bw: f64, bh: f64) -> bool {
    ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Flappy bird", [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| {
                panic!("Failed to build PistonWindow: {}", e)
            });

    let mut rng = rand::rng();

    // Create several pipes spaced apart horizontally.
    let pipe_count = 3;
    let mut pipes: Vec<Pipe> = (0..pipe_count)
        .map(|i| Pipe {
            x: WINDOW_WIDTH + i as f64 * 200.0,
            gap_y: rng.random_range(GAP_SIZE / 2.0..(WINDOW_HEIGHT - GAP_SIZE / 2.0)),
        })
        .collect();

    // Player's block (bird) starting position.
    let mut block_x = 50.0;
    let mut block_y = WINDOW_HEIGHT / 2.0;
    let mut vel_y = 0.0; // Vertical velocity of the bird.
    let mut game_over = false;

    while let Some(event) = window.next() {
        // If game is not over, update the game state.
        if !game_over {
            if let Some(_args) = event.update_args() {
                block_y += vel_y;
                vel_y += GRAVITY;
            }

            // Move pipes to the left.
            for pipe in pipes.iter_mut() {
                pipe.x -= PIPE_SPEED;
                // Reset when off screen.
                if pipe.x < -BLOCK_SIZE {
                    pipe.x = WINDOW_WIDTH;
                    pipe.gap_y = rng.random_range(GAP_SIZE / 2.0..(WINDOW_HEIGHT - GAP_SIZE / 2.0));
                }
            }

            // Check for collisions.
            for pipe in &pipes {
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

                // Check collision with bottom pipe.
                if rects_intersect(block_x,
                                   block_y,
                                   BLOCK_SIZE,
                                   BLOCK_SIZE,
                                   pipe.x,
                                   0.0,
                                   BLOCK_SIZE,
                                   bottom_height) {
                    game_over = true;
                }
                // Check collision with top pipe.
                if rects_intersect(block_x,
                                   block_y,
                                   BLOCK_SIZE,
                                   BLOCK_SIZE,
                                   pipe.x,
                                   pipe.gap_y + GAP_SIZE / 2.0,
                                   BLOCK_SIZE,
                                   top_height) {
                    game_over = true;
                }
            }

            // Check collision with floor or ceiling.
            if block_y < 0.0 || block_y + BLOCK_SIZE > WINDOW_HEIGHT {
                game_over = true;
            }
        }

        // Restart logic: press R to restart when game is over.
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if key == Key::R && game_over {
                // Reset game state.
                block_x = 50.0;
                block_y = WINDOW_HEIGHT / 2.0;
                vel_y = 0.0;
                for (i, pipe) in pipes.iter_mut().enumerate() {
                    pipe.x = WINDOW_WIDTH + i as f64 * 200.0;
                    pipe.gap_y = rng.random_range(GAP_SIZE / 2.0..(WINDOW_HEIGHT - GAP_SIZE / 2.0));
                }
                game_over = false;
            }
            // When up is pressed, the bird jumps.
            if !game_over {
                match key {
                    Key::Up => {
                        vel_y = JUMP_IMPULSE;
                    },
                    Key::Down => {
                        block_y += STEP;
                    },
                    Key::Left => block_x -= STEP,
                    Key::Right => block_x += STEP,
                    _ => {}
                }
            }
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);

            // Flip coordinate system: origin is bottom left.
            let flipped_transform = c.transform
                .trans(0.0, WINDOW_HEIGHT)
                .scale(1.0, -1.0);

            // Draw pipes.
            for pipe in &pipes {
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

            // Draw the player's block (bird) as a blue square.
            rectangle(
                [0.0, 0.0, 1.0, 1.0],
                [block_x, block_y, BLOCK_SIZE, BLOCK_SIZE],
                c.transform,
                g,
            );
        });
    }
}