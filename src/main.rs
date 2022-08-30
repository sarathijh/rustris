use tetris::{game::tetris, piece::PieceType, random::RandomBag, srs::SrsPieceSet};

mod tetris;

fn main() {
    tetris(
        SrsPieceSet {},
        RandomBag::new(vec![
            PieceType::I,
            PieceType::T,
            PieceType::O,
            PieceType::J,
            PieceType::L,
            PieceType::Z,
            PieceType::S,
        ]),
    )
}
