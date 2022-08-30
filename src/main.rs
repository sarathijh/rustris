use tetris::{
    game::tetris, input::DasInputActions, piece::PieceType, queue::Queue, random::RandomBag,
    srs::SrsPieceSet,
};

mod tetris;

fn main() {
    tetris(
        SrsPieceSet::new(),
        Queue::new(5, RandomBag::new(PieceType::all())),
        DasInputActions::new(0.18333333333, 0.03333333333),
    )
}
