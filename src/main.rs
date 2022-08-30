use tetris::{
    game::tetris, input::DasInputActions, keyboard_query::KeyboardQueryInputSource,
    piece::PieceType, queue::Queue, random::RandomBag, srs::SrsPieceSet, termion::TermionRenderer,
};

mod tetris;

fn main() {
    tetris(
        SrsPieceSet::new(),
        Queue::new(5, RandomBag::new(PieceType::all())),
        DasInputActions::new(
            KeyboardQueryInputSource::new(),
            0.18333333333,
            0.03333333333,
        ),
        TermionRenderer::new(),
    )
}
