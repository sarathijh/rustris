extern crate tetris_core;

use game_loop::game_loop;
use tetris_core::{
    game::TetrisGame, input::DasInputActions, piece::PieceType, queue::Queue, random::RandomBag,
};
use tetris_keyboard_query::KeyboardQueryInputSource;
use tetris_srs::SrsPieceSet;
use tetris_termion::TermionRenderer;

fn main() {
    let mut game = TetrisGame::new(
        SrsPieceSet::new(),
        Queue::new(5, RandomBag::new(PieceType::all())),
        DasInputActions::new(
            KeyboardQueryInputSource::new(),
            0.18333333333,
            0.03333333333,
        ),
        TermionRenderer::new(),
    );

    game.start();

    game_loop(
        game,
        60,
        1.0,
        |g| g.game.update(g.fixed_time_step()),
        |g| g.game.render(),
    );
}
