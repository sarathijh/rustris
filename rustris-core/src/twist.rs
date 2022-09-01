use crate::{
    board::Board,
    piece::{Piece, PieceSet, PieceType},
    position::Position,
};

pub trait TwistDetector<TPieceSet: PieceSet> {
    fn is_twist(&self, board: &Board, piece_set: &TPieceSet, piece: &Piece) -> bool;
}

pub struct ThreeCornerTTwistDetector;
impl<TPieceSet: PieceSet> TwistDetector<TPieceSet> for ThreeCornerTTwistDetector {
    fn is_twist(&self, board: &Board, _: &TPieceSet, piece: &Piece) -> bool {
        if piece.piece_type == PieceType::T {
            let mut corner_count = 0;

            if board.is_filled(piece.position + Position::new(-1, 1)) {
                corner_count += 1;
            }

            if board.is_filled(piece.position + Position::new(1, 1)) {
                corner_count += 1;
            }

            if board.is_filled(piece.position + Position::new(-1, -1)) {
                corner_count += 1;
            }

            if board.is_filled(piece.position + Position::new(1, -1)) {
                corner_count += 1;
            }

            return corner_count >= 3;
        }

        return false;
    }
}

pub struct AllTwistDetector;
impl<TPieceSet: PieceSet> TwistDetector<TPieceSet> for AllTwistDetector {
    fn is_twist(&self, board: &Board, piece_set: &TPieceSet, piece: &Piece) -> bool {
        let directions = [
            Position::left(),
            Position::right(),
            Position::up(),
            Position::down(),
        ];

        return directions.iter().all(|direction| {
            board.is_obstructed(
                piece_set.units(&piece.piece_type, &piece.rotation),
                piece.position + *direction,
            )
        });
    }
}
