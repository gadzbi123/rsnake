use piston_window::*;
use rand::Rng;

use crate::colors;
use crate::draw::*;
use crate::history::History;
use crate::physics::{Direction, Position};
use crate::snake::Snake;

const FPS: f64 = 10.0;
// const RESTART_TIME: f64 = 1.0;

fn fps_in_ms(fps: f64) -> f64 {
    1.0 / fps
}

fn calc_random_pos(width: u32, height: u32) -> Position {
    let mut rng = rand::thread_rng();

    Position {
        x: rng.gen_range(0, width as i32),
        y: rng.gen_range(0, height as i32),
    }
}

pub struct Game {
    snake: Snake,
    fruit: Position,
    size: (u32, u32),
    waiting_time: f64,
    score: u32,
    history: History,
    over: bool,
    paused: bool,
    replay: bool,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        // use fn defined at eof to calc random fruit / snake pos here
        let random_snake_pos = calc_random_pos(width, height);
        let random_fruit_pos = calc_random_pos(width, height);
        Self {
            snake: Snake::new(random_snake_pos.clone()),
            fruit: random_fruit_pos.clone(),
            size: (width, height),
            waiting_time: 0.0,
            score: 0,
            history: History::new(random_snake_pos, random_fruit_pos),
            over: false,
            paused: true,
            replay: false,
        }
    }

    pub fn start(&mut self) {
        self.paused = false;
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    // pub fn toggle_game_state(&mut self) {
    //     if self.paused {
    //         self.start();
    //     } else {
    //         self.pause();
    //     }
    // }

    pub fn draw(&self, ctx: Context, g: &mut G2d) {
        draw_block(&ctx, g, colors::FRUIT, &self.fruit);
        self.snake.draw(&ctx, g);
        // draw_text(&ctx, g, colors::SCORE, self.score.to_string());

        if self.over {
            draw_overlay(&ctx, g, colors::OVERLAY, self.size)
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.waiting_time > fps_in_ms(FPS) && self.over && self.replay {
            println!("REPLAY");
            todo!("same as on the bottom");
        }

        if self.waiting_time > fps_in_ms(FPS) && !self.over && !self.paused {
            self.waiting_time = 0.0;

            if !self.snake.is_tail_overlapping() && !self.snake.will_tail_overlap() {
                self.snake.update(self.size.0, self.size.1);

                if *self.snake.get_head_pos() == self.fruit {
                    self.snake.grow();
                    self.snake.update(self.size.0, self.size.1);
                    self.fruit = calc_random_pos(self.size.0, self.size.1);
                    self.calc_score();
                }
            } else {
                self.over = true;
            }
        }
    }

    pub fn key_down(&mut self, key: keyboard::Key) {
        use keyboard::Key;

        // match key {
        //     Key::R => self.over = false, // temp solution -> replace current game state trough new one
        //     Key::Space => self.toggle_game_state(),
        //     _ => self.start(),
        // }

        match key {
            Key::A | Key::Left => self.snake.set_dir(Direction::Left),
            Key::W | Key::Up => self.snake.set_dir(Direction::Up),
            Key::D | Key::Right => self.snake.set_dir(Direction::Right),
            Key::S | Key::Down => self.snake.set_dir(Direction::Down),
            Key::R => self.play_replay(),
            _ => {}
        }
    }

    pub fn play_replay(&mut self) {
        if self.over {
            self.replay = true
        };
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    fn calc_score(&mut self) {
        self.score = (self.snake.get_len() * 10) as u32
    }

    // IMPORTANT!! -

    // fn update_snake(&mut self, dir: Option<Direction>) {
    //     if self.check_if_snake_alive(dir) {
    //         self.snake.move_forward(dir);
    //         self.check_eating();
    //     } else {
    //         self.game_over = true;
    //     }
    //     self.waiting_time = 0.0;
    // }
}
