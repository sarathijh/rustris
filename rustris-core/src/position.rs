use std::ops;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn left() -> Self {
        Self { x: -1, y: 0 }
    }

    pub fn right() -> Self {
        Self { x: 1, y: 0 }
    }

    pub fn up() -> Self {
        Self { x: 0, y: 1 }
    }

    pub fn down() -> Self {
        Self { x: 0, y: -1 }
    }
}

impl ops::Mul<i32> for Position {
    type Output = Position;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Mul<Position> for i32 {
    type Output = Position;

    fn mul(self, rhs: Position) -> Self::Output {
        Position::new(rhs.x * self, rhs.y * self)
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        *self = Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::SubAssign<Position> for Position {
    fn sub_assign(&mut self, rhs: Position) {
        *self = Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
