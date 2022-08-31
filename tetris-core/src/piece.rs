use super::{board::Board, position::Position};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PieceType {
    I,
    T,
    O,
    J,
    L,
    Z,
    S,
}

impl PieceType {
    pub fn all() -> Vec<PieceType> {
        vec![
            PieceType::I,
            PieceType::T,
            PieceType::O,
            PieceType::J,
            PieceType::L,
            PieceType::Z,
            PieceType::S,
        ]
    }
}

pub enum Direction {
    CW,
    CCW,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Rotation {
    Up,
    Right,
    Down,
    Left,
}

impl Rotation {
    pub fn rotate(&self, direction: Direction) -> Rotation {
        match direction {
            Direction::CW => match self {
                Rotation::Up => Rotation::Right,
                Rotation::Right => Rotation::Down,
                Rotation::Down => Rotation::Left,
                Rotation::Left => Rotation::Up,
            },
            Direction::CCW => match self {
                Rotation::Up => Rotation::Left,
                Rotation::Right => Rotation::Up,
                Rotation::Down => Rotation::Right,
                Rotation::Left => Rotation::Down,
            },
        }
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub rotation: Rotation,
    pub position: Position,
}

pub trait PieceSet {
    fn units(&self, piece_type: &PieceType, rotation: &Rotation) -> [Position; 4];
    fn rotate_piece(&self, board: &Board, piece: &Piece, direction: Direction) -> Option<Piece>;
}
