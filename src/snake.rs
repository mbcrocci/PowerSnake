use crate::{Position, GRID_SIZE, ModuloSigned, GameResult, Context, graphics};
use std::collections::VecDeque;


#[derive(Clone, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Snake {
    bits: Vec<Position>,
    pub is_alive: bool,
    queued_grows: VecDeque<bool>,
    direction: Direction,
}

impl Snake {
    pub fn new(start_rect: Position) -> Self {
        let mut snake = Snake {
            bits: Vec::new(),
            is_alive: true,
            queued_grows: VecDeque::new(),
            direction: Direction::Right,
        };

        snake.bits.push(start_rect);
        snake
    }

    pub fn kill(&mut self) {
        self.is_alive = false;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        let new_direction = match (direction.clone(), self.direction.clone()) {
            (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up) => self.direction.clone(),
            (_, _) => direction,
        };

        self.direction = new_direction;
    }

    pub fn grow(&mut self) {
        self.queued_grows.push_back(true);
    }

    pub fn check_collison(&self, r: &Position) -> bool {
        &self.bits[0] == r
    }

    pub fn check_self_collision(&self) -> bool {
        for (i, bit) in self.bits.iter().enumerate() {
            if i > 0 {
                if self.check_collison(bit) {
                    return true;
                }
            }
        }
        false
    }

    pub fn update(&mut self) {
        let previous_state = self.bits.to_vec();

        for (i, bit) in &mut self.bits.iter_mut().enumerate() {
            if i > 0 {
                bit.x = previous_state[i - 1].x;
                bit.y = previous_state[i - 1].y;
            } else {
                match self.direction {
                    Direction::Left => bit.x = (bit.x - 1).modulo(GRID_SIZE.0),
                    Direction::Right => bit.x = (bit.x + 1).modulo(GRID_SIZE.0),
                    Direction::Up => bit.y = (bit.y - 1).modulo(GRID_SIZE.1),
                    Direction::Down => bit.y = (bit.y + 1).modulo(GRID_SIZE.1),
                };
            }
        }

        // Can only grow once per update
        if let Some(_) = self.queued_grows.pop_front() {
            let last = previous_state.last().unwrap();
            self.bits.push((*last).clone());
        }

        if self.check_self_collision() {
            self.kill();
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for (i, r) in self.bits.iter().enumerate() {
            let color = if i == 0 {
                [1.0, 1.0, 1.0, 1.0]
            } else {
                [1.0, 1.0, 0.0, 1.0]
            };

            let rm = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                r.clone().into(),
                color.into(),
            )?;

            graphics::draw(ctx, &rm, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }
        Ok(())
    }
}

