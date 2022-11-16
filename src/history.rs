use crate::physics::Position;
use crate::snake::{self, Snake, INITIAL_SNAKE_TAIL_LENGTH};
use std::collections::LinkedList;
use std::error::Error;
pub struct History {
    pub starting_snake: Snake,
    pub heads_pos: LinkedList<Position>,
    pub fruits_pos: LinkedList<Position>,
    pub tail_current_len: usize,
    pub init: bool,
    fp: LinkedList<Position>,
    hp: LinkedList<Position>,
}

impl History {
    pub fn new(snake: Snake, first_fruit_pos: Position) -> Self {
        Self {
            starting_snake: snake,
            heads_pos: LinkedList::new(),
            tail_current_len: INITIAL_SNAKE_TAIL_LENGTH,
            fruits_pos: LinkedList::from([first_fruit_pos]),
            init: false,
            fp: LinkedList::new(),
            hp: LinkedList::new(),
        }
    }

    pub fn add_head(&mut self, head_pos: Position) {
        self.heads_pos.push_back(head_pos);
    }

    pub fn add_fruit(&mut self, fruit_pos: Position) {
        self.fruits_pos.push_back(fruit_pos);
    }

    pub fn get_latest_fruit(&mut self) -> Option<Position> {
        if !self.fruits_pos.is_empty() {
            self.fp
                .push_back(self.fruits_pos.front().unwrap().to_owned());
        }
        self.fruits_pos.pop_front()
    }
    pub fn get_latest_head(&mut self) -> Option<Position> {
        if !self.heads_pos.is_empty() {
            self.hp
                .push_back(self.heads_pos.front().unwrap().to_owned());
        }
        self.heads_pos.pop_front()
    }

    pub fn rewind_replay(&mut self) {
        self.heads_pos.append(&mut self.hp);
        self.fruits_pos.append(&mut self.fp);
        self.init = false;
    }

    pub fn get_start_snake(&self) -> Snake {
        self.starting_snake.clone()
    }
}
