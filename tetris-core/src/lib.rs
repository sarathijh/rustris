use game::TetrisGame;
use game_loop::game_loop;
use input::InputActions;
use piece::{PieceSet, PieceType};
use queue::Queue;
use random::Random;
use renderer::Renderer;

pub mod board;
mod game;
pub mod input;
pub mod piece;
pub mod position;
pub mod queue;
pub mod random;
pub mod renderer;

pub fn tetris_game<
    TPieceSet: PieceSet,
    TRandom: Random<PieceType>,
    TInputActions: InputActions,
    TRenderer: Renderer<TPieceSet>,
>(
    piece_set: TPieceSet,
    queue: Queue<PieceType, TRandom>,
    input_actions: TInputActions,
    renderer: TRenderer,
) {
    let mut game = TetrisGame::new(piece_set, queue, input_actions, renderer);

    game.start();

    game_loop(
        game,
        60,
        1.0,
        |g| g.game.update(g.fixed_time_step()),
        |g| g.game.render(),
    );
}
