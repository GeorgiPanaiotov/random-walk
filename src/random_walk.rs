use nannou::prelude::*;

#[derive(Clone, Copy)]
pub enum Direction {
    UP = 1,
    DOWN = 2,
    LEFT = 3,
    RIGHT = 4,
}
pub struct RandomWalk {
    pub pos: Vec2,
}

impl RandomWalk {
    pub fn new() -> Self {
        Self {
            pos: vec2(0.0, 0.0),
        }
    }

    pub fn update(&mut self, dir: Direction) {
        match dir {
            Direction::UP => self.pos.y += 1.0,
            Direction::DOWN => self.pos.y -= 1.0,
            Direction::LEFT => self.pos.x -= 1.0,
            Direction::RIGHT => self.pos.x += 1.0,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.rect().w_h(2.0, 2.0).xy(self.pos).color(WHITE);
    }
}
