use std::ops::{ Add, Sub, AddAssign, SubAssign, Mul,MulAssign, Div, DivAssign, Neg };

#[derive(Debug, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vec2 {
    pub fn new(x: f64, y:f64) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalise(&self) -> Option<Self> {
        let length = self.length();
        if length == 0.0 {
            return None
        }

        Some(Self {x: self.x / length, y: self.y / length})
    }

    pub fn distance_to_squared(&self, rhs: Self) -> f64 {
        let x_component = self.x - rhs.x;
        let y_component = self.y - rhs.y;
        x_component * x_component + y_component * y_component
    }

    pub fn distance_to(&self, rhs: Self) -> f64 {
        self.distance_to_squared(rhs).sqrt()
    }
}
