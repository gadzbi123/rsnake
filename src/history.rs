use crate::physics::Position;
use crate::snake::{self, Snake, INITIAL_SNAKE_TAIL_LENGTH};
use std::collections::LinkedList;
pub struct History {
    pub starting_snake: Snake,
    pub next_head_positions: LinkedList<Position>,
    pub fruits_pos: LinkedList<Position>,
    pub tail_current_len: usize,
    pub init: bool,
}

impl History {
    pub fn new(snake: Snake, first_fruit_pos: Position) -> Self {
        Self {
            starting_snake: snake,
            next_head_positions: LinkedList::new(),
            tail_current_len: INITIAL_SNAKE_TAIL_LENGTH,
            fruits_pos: LinkedList::from([first_fruit_pos]),
            init: false,
        }
    }

    pub fn add_head(&mut self, head_pos: Position) {
        self.next_head_positions.push_back(head_pos);
    }

    pub fn add_fruit(&mut self, fruit_pos: Position) {
        self.fruits_pos.push_back(fruit_pos);
    }

    pub fn get_latest_fruit(&mut self) -> Position {
        self.fruits_pos.pop_front().unwrap()
    }
    pub fn get_latest_head(&mut self) -> Position {
        self.next_head_positions.pop_front().unwrap()
    }

    pub fn get_start_snake(&self) -> Snake {
        self.starting_snake.clone()
    }
}
