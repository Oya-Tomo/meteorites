use std::ops::Add;

use rand::{self, Rng};

pub const GRAVITY: f32 = 0.0098;

pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    fn add_with_ratio(&self, self_ratio: f32, rhs: Velocity, rhs_ratio: f32) -> Velocity {
        return Velocity {
            x: (self.x * self_ratio + rhs.x * rhs_ratio) / self_ratio + rhs_ratio,
            y: (self.y * self_ratio + rhs.y * rhs_ratio) / self_ratio + rhs_ratio,
        };
    }
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Meteorite {
    pub velocity: Velocity,
    pub position: Position,
    pub size: f32,
}

impl Meteorite {
    pub fn random(view_size: (isize, isize)) -> Self {
        let mut rng = rand::thread_rng();
        let px = rng.gen_range(-view_size.0 / 2, view_size.0 / 2) as f32;
        let py = rng.gen_range(-view_size.1 / 2, view_size.1 / 2) as f32;
        let vx = rng.gen_range(-5, 5) as f32 / 10.0;
        let vy = rng.gen_range(-5, 5) as f32 / 10.0;

        Self {
            velocity: Velocity { x: vx, y: vy },
            position: Position { x: px, y: py },
            size: 4.0,
        }
    }

    pub fn update(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }
}

impl Add for Meteorite {
    type Output = Meteorite;

    fn add(self, rhs: Self) -> Self::Output {
        return Meteorite {
            velocity: self
                .velocity
                .add_with_ratio(self.size, rhs.velocity, rhs.size),
            position: self.position,
            size: self.size + rhs.size,
        };
    }
}
