extern crate rustris_core;

use game_loop::game_loop;
use rustris_core::{
    game::Rustris,
    input::DasInputActions,
    piece::PieceType,
    queue::Queue,
    random::RandomBag,
    twist::{AllTwistDetector, ThreeCornerTTwistDetector},
};
use rustris_keyboard_query::KeyboardQueryInputSource;
use rustris_srs::SrsPieceSet;
use rustris_termion::TermionRenderer;

fn main() {
    // Create a new rustris game simulation using:
    // - Super Rotation System (SRS)
    // - A next queue showing 5 pieces
    // - A random bag generator
    // - Delayed Auto Shift (DAS) input
    // - An input source implementation that uses the keyboard_query crate
    // - A rendering implementation that uses the termion crate
    let mut game = Rustris::new(
        SrsPieceSet,
        Queue::new(5, RandomBag::new(PieceType::all())),
        DasInputActions::new(
            KeyboardQueryInputSource::new(),
            0.18333333333,
            0.03333333333,
        ),
        AllTwistDetector,
        TermionRenderer::new(),
    );

    // Initialize the simulation
    game.init();

    // Start a game loop that updates and renders the simulation
    // - Uses the game_loop crate
    game_loop(
        game,
        60,
        1.0,
        |g| g.game.update(g.fixed_time_step()),
        |g| g.game.render(g.fixed_time_step()),
    );
}
