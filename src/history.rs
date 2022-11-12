use crate::physics::Position;
use crate::snake::INITIAL_SNAKE_TAIL_LENGTH;
pub struct History {
    pub head_positions: Vec<Position>,
    pub tail_change_len_pos: Vec<Position>,
    pub tail_current_len: usize,
}

impl History {
    pub fn new(head_pos: Position, first_fruit_pos: Position) -> Self {
        Self {
            head_positions: vec![head_pos],
            tail_current_len: INITIAL_SNAKE_TAIL_LENGTH,
            tail_change_len_pos: vec![first_fruit_pos],
        }
    }

    pub fn add_head(&mut self, head_pos: Position) {
        self.head_positions.push(head_pos);
    }

    pub fn add_fruit(&mut self, fruit_pos: Position) {
        self.tail_change_len_pos.push(fruit_pos);
    }
}
