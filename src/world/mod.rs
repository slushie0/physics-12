use macroquad::color::Color;
use std::f64::consts::PI;
use std::ops::{Add, Sub, AddAssign, SubAssign};

pub const G: f64 = 6.67430e-11;
pub const Ke: f64 = 9e9;

#[derive(Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}
impl Vec2 {
    pub const ZERO: Self = Self { x: 0., y: 0. };
}

pub struct GameState {
    pub bodies: Vec<Body>,
}

pub struct Body {
    pub name: String,

    pub pos: Vec2,
    pub vel: Vec2,

    pub mass: f64,
    pub radius: f64,

    pub charge: f64,
    pub color: Color,
}

#[derive(Clone)]
pub struct Force {
    pub x: f64,
    pub y: f64,
}

impl Force {
    pub fn from_components(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn from_polar(magnitude: f64, direction_deg: f64) -> Self {
        let rad = direction_deg.to_radians();
        Self {
            x: magnitude * rad.cos(),
            y: magnitude * rad.sin(),
        }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn direction_deg(&self) -> f64 {
        self.y.atan2(self.x) * 180.0 / PI
    }
}
impl Add for Force {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
       Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Force {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
       Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl AddAssign for Force {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl SubAssign for Force {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

pub fn distance(body1: &Body, body2: &Body) -> f64 {
    let dx = body1.pos.x - body2.pos.x;
    let dy = body1.pos.y - body2.pos.y;
    (dx * dx + dy * dy).sqrt()
}
