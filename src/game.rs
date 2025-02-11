use rand::Rng;

// Constants
pub const BACK_COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
pub const BLOCK_SIZE: f64 = 25.0;

pub const GAP_SIZE: f64 = 90.0;
pub const WINDOW_HEIGHT: f64 = 360.0;
pub const WINDOW_WIDTH: f64 = 640.0;
pub const PIPE_SPEED: f64 = 2.0;
pub const GRAVITY: f64 = 0.2;
pub const JUMP_IMPULSE: f64 = -5.0;

pub struct Pipe {
    pub x: f64,
    pub gap_y: f64,
}

pub struct Game{
    pub pipes: Vec<Pipe>,
    pub block_x: f64,
    pub block_y: f64,
    pub vel_y: f64,
    pub game_over: bool,
}

impl Game {
    pub fn new(pipe_count: i32) -> Game {
        let mut rng = rand::rng();
        let pipes: Vec<Pipe> = (0..pipe_count).map(
            |i| Pipe {
                x: WINDOW_WIDTH + i as f64 * 200.0,
                gap_y: rng.random_range(GAP_SIZE / 2.0..(WINDOW_HEIGHT - GAP_SIZE / 2.0)),
            }
        ).collect();

        Game {
            pipes,
            block_x: 50.0,
            block_y: WINDOW_HEIGHT / 2.0,
            vel_y: 0.0,
            game_over: false,
        }
    }

    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        self.block_y += self.vel_y;
        self.vel_y += GRAVITY;

        for pipe in self.pipes.iter_mut() {
            pipe.x -= PIPE_SPEED;
            if pipe.x < -BLOCK_SIZE {
                pipe.x = WINDOW_WIDTH;
                pipe.gap_y = rand::random_range(GAP_SIZE / 2.0..(WINDOW_HEIGHT - GAP_SIZE / 2.0));
            }
        }

        // Check for collisions.
        self.check_collisions();
    }

    pub fn jump(&mut self) {
        if !self.game_over {
            self.vel_y = JUMP_IMPULSE;
        }
    }

    pub fn reset(&mut self) {
        if self.game_over {
            let mut rng = rand::rng();
            self.block_x = 50.0;
            self.block_y = WINDOW_HEIGHT / 2.0;
            self.vel_y = 0.0;
            for (i, pipe) in self.pipes.iter_mut().enumerate() {
                pipe.x = WINDOW_WIDTH + i as f64 * 200.0;
                pipe.gap_y = rng.random_range(GAP_SIZE / 2.0..(WINDOW_HEIGHT - GAP_SIZE / 2.0));
            }
            self.game_over = false;
        }
    }

    fn check_collisions(&mut self) {
        for pipe in &self.pipes {
            // Calculate the heights of the top and bottom pipes
            let bottom_pipe_height = pipe.gap_y - GAP_SIZE / 2.0;
            let top_pipe_y = pipe.gap_y + GAP_SIZE / 2.0;
            let top_pipe_height = WINDOW_HEIGHT - top_pipe_y;

            // Check collision with bottom pipe
            if bottom_pipe_height > 0.0 {
                if rects_intersect(
                    self.block_x, self.block_y, BLOCK_SIZE, BLOCK_SIZE,
                    pipe.x, 0.0, BLOCK_SIZE, bottom_pipe_height
                ) {
                    self.game_over = true;
                }
            }
            
            // Check collision with top pipe
            if top_pipe_height > 0.0 {
                if rects_intersect(
                    self.block_x, self.block_y, BLOCK_SIZE, BLOCK_SIZE,
                    pipe.x, top_pipe_y, BLOCK_SIZE, top_pipe_height
                ) {
                    self.game_over = true;
                }
            }
        }

        // Check floor/ceiling collisions
        if self.block_y < 0.0 || self.block_y + BLOCK_SIZE > WINDOW_HEIGHT {
            self.game_over = true;
        }
    }
}

fn rects_intersect(ax: f64, ay: f64, aw: f64, ah: f64, bx: f64, by: f64, bw: f64, bh: f64) -> bool {
    ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
}