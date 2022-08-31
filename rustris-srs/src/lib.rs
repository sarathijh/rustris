use rustris_core::{
    board::Board,
    piece::{Direction, Piece, PieceSet, PieceType, Rotation},
    position::Position,
};

pub struct SrsPieceSet {}

impl SrsPieceSet {
    pub fn new() -> Self {
        Self {}
    }
}

impl PieceSet for SrsPieceSet {
    fn units(&self, piece_type: &PieceType, rotation: &Rotation) -> [Position; 4] {
        match piece_type {
            PieceType::I => units_i(rotation),
            PieceType::T => units_t(rotation),
            PieceType::O => units_o(rotation),
            PieceType::J => units_j(rotation),
            PieceType::L => units_l(rotation),
            PieceType::Z => units_z(rotation),
            PieceType::S => units_s(rotation),
        }
    }

    fn rotate_piece(&self, board: &Board, piece: &Piece, direction: Direction) -> Option<Piece> {
        let target_rotation = &piece.rotation.rotate(direction);

        let kick_offsets_a = kick_offsets(&piece.piece_type, &piece.rotation);
        let kick_offsets_b = kick_offsets(&piece.piece_type, target_rotation);

        for (kick_offset_a, kick_offset_b) in kick_offsets_a.iter().zip(kick_offsets_b.iter()) {
            let target_kick_offset = *kick_offset_a - *kick_offset_b;
            let target_position = piece.position + target_kick_offset;

            if !board.is_obstructed(
                self.units(&piece.piece_type, target_rotation),
                &target_position,
            ) {
                return Some(Piece {
                    piece_type: piece.piece_type,
                    rotation: *target_rotation,
                    position: target_position,
                });
            }
        }

        return None;
    }
}

fn units_i(rotation: &Rotation) -> [Position; 4] {
    match rotation {
        Rotation::Up => [
            Position::new(-1, 0),
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(2, 0),
        ],
        Rotation::Right => [
            Position::new(0, -2),
            Position::new(0, -1),
            Position::new(0, 0),
            Position::new(0, 1),
        ],
        Rotation::Down => [
            Position::new(-2, 0),
            Position::new(-1, 0),
            Position::new(0, 0),
            Position::new(1, 0),
        ],
        Rotation::Left => [
            Position::new(0, -1),
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(0, 2),
        ],
    }
}

fn units_t(rotation: &Rotation) -> [Position; 4] {
    match rotation {
        Rotation::Up => [
            Position::new(0, 0),
            Position::new(-1, 0),
            Position::new(1, 0),
            Position::new(0, 1),
        ],
        Rotation::Right => [
            Position::new(0, 0),
            Position::new(0, -1),
            Position::new(1, 0),
            Position::new(0, 1),
        ],
        Rotation::Down => [
            Position::new(0, 0),
            Position::new(-1, 0),
            Position::new(1, 0),
            Position::new(0, -1),
        ],
        Rotation::Left => [
            Position::new(0, 0),
            Position::new(-1, 0),
            Position::new(0, 1),
            Position::new(0, -1),
        ],
    }
}

fn units_o(rotation: &Rotation) -> [Position; 4] {
    match rotation {
        Rotation::Up => [
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(0, 1),
            Position::new(1, 1),
        ],
        Rotation::Right => [
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(0, 1),
            Position::new(1, 1),
        ],
        Rotation::Down => [
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(0, 1),
            Position::new(1, 1),
        ],
        Rotation::Left => [
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(0, 1),
            Position::new(1, 1),
        ],
    }
}

fn units_j(rotation: &Rotation) -> [Position; 4] {
    match rotation {
        Rotation::Up => [
            Position::new(0, 0),
            Position::new(-1, 0),
            Position::new(1, 0),
            Position::new(-1, 1),
        ],
        Rotation::Right => [
            Position::new(0, -1),
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(1, 1),
        ],
        Rotation::Down => [
            Position::new(0, 0),
            Position::new(-1, 0),
            Position::new(1, 0),
            Position::new(1, -1),
        ],
        Rotation::Left => [
            Position::new(0, -1),
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(-1, -1),
        ],
    }
}

fn units_l(rotation: &Rotation) -> [Position; 4] {
    match rotation {
        Rotation::Up => [
            Position::new(0, 0),
            Position::new(-1, 0),
            Position::new(1, 0),
            Position::new(1, 1),
        ],
        Rotation::Right => [
            Position::new(0, -1),
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(1, -1),
        ],
        Rotation::Down => [
            Position::new(0, 0),
            Position::new(-1, 0),
            Position::new(1, 0),
            Position::new(-1, -1),
        ],
        Rotation::Left => [
            Position::new(0, -1),
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(-1, 1),
        ],
    }
}

fn units_z(rotation: &Rotation) -> [Position; 4] {
    match rotation {
        Rotation::Up => [
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(-1, 1),
            Position::new(0, 1),
        ],
        Rotation::Right => [
            Position::new(0, 0),
            Position::new(0, -1),
            Position::new(1, 0),
            Position::new(1, 1),
        ],
        Rotation::Down => [
            Position::new(0, 0),
            Position::new(-1, 0),
            Position::new(1, -1),
            Position::new(0, -1),
        ],
        Rotation::Left => [
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(-1, 0),
            Position::new(-1, -1),
        ],
    }
}

fn units_s(rotation: &Rotation) -> [Position; 4] {
    match rotation {
        Rotation::Up => [
            Position::new(0, 0),
            Position::new(-1, 0),
            Position::new(0, 1),
            Position::new(1, 1),
        ],
        Rotation::Right => [
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(1, 0),
            Position::new(1, -1),
        ],
        Rotation::Down => [
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(-1, -1),
            Position::new(0, -1),
        ],
        Rotation::Left => [
            Position::new(0, 0),
            Position::new(0, -1),
            Position::new(-1, 0),
            Position::new(-1, 1),
        ],
    }
}

fn kick_offsets(piece_type: &PieceType, rotation: &Rotation) -> Vec<Position> {
    match piece_type {
        PieceType::I => match rotation {
            Rotation::Up => vec![
                Position::new(0, 0),
                Position::new(-1, 0),
                Position::new(2, 0),
                Position::new(-1, 0),
                Position::new(2, 0),
            ],
            Rotation::Right => vec![
                Position::new(-1, 0),
                Position::new(0, 0),
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, -2),
            ],
            Rotation::Down => vec![
                Position::new(-1, 1),
                Position::new(1, 1),
                Position::new(-2, 1),
                Position::new(1, 0),
                Position::new(-2, 0),
            ],
            Rotation::Left => vec![
                Position::new(0, 1),
                Position::new(0, 1),
                Position::new(0, 1),
                Position::new(0, -1),
                Position::new(0, 2),
            ],
        },
        PieceType::T | PieceType::Z | PieceType::S | PieceType::L | PieceType::J => {
            match rotation {
                Rotation::Up => vec![
                    Position::new(0, 0),
                    Position::new(0, 0),
                    Position::new(0, 0),
                    Position::new(0, 0),
                    Position::new(0, 0),
                ],
                Rotation::Right => vec![
                    Position::new(0, 0),
                    Position::new(1, 0),
                    Position::new(1, -1),
                    Position::new(0, 2),
                    Position::new(1, 2),
                ],
                Rotation::Down => vec![
                    Position::new(0, 0),
                    Position::new(0, 0),
                    Position::new(0, 0),
                    Position::new(0, 0),
                    Position::new(0, 0),
                ],
                Rotation::Left => vec![
                    Position::new(0, 0),
                    Position::new(-1, 0),
                    Position::new(-1, -1),
                    Position::new(0, 2),
                    Position::new(-1, 2),
                ],
            }
        }
        PieceType::O => match rotation {
            Rotation::Up => vec![Position::new(0, 0)],
            Rotation::Right => vec![Position::new(0, 0)],
            Rotation::Down => vec![Position::new(0, 0)],
            Rotation::Left => vec![Position::new(0, 0)],
        },
    }
}
