use std::ops;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Position {
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
