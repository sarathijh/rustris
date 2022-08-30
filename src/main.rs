use tetris::{game::tetris, piece::PieceType, queue::Queue, random::RandomBag, srs::SrsPieceSet};

mod tetris;

fn main() {
    tetris(
        SrsPieceSet {},
        Queue::new(
            5,
            RandomBag::new(vec![
                PieceType::I,
                PieceType::T,
                PieceType::O,
                PieceType::J,
                PieceType::L,
                PieceType::Z,
                PieceType::S,
            ]),
        ),
    )
}
