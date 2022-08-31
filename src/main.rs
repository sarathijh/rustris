extern crate tetris_core;

use tetris_core::{
    input::DasInputActions, piece::PieceType, queue::Queue, random::RandomBag, tetris_game,
};
use tetris_keyboard_query::KeyboardQueryInputSource;
use tetris_srs::SrsPieceSet;
use tetris_termion::TermionRenderer;

fn main() {
    tetris_game(
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
