use std::io::{stdout, Stdout};

use game_loop::game_loop;
use ndarray::Array2;
use std::io::Write;
use termion::{
    clear::All,
    cursor::{Goto, Hide},
    event::Key,
    input::{Keys, TermRead},
    raw::{IntoRawMode, RawTerminal},
    AsyncReader,
};

use super::{
    board::Board,
    input::{Action, InputActions},
    piece::{Direction, Piece, PieceSet, PieceType, Rotation},
    position::Position,
    queue::Queue,
    random::Random,
};

const BOARD_WIDTH: usize = 10;
const BOARD_VISIBLE_HEIGHT: usize = 20;

#[derive(Clone, Copy)]
enum FillType {
    Empty,
    Unit,
    Ghost,
}

pub fn tetris<TPieceSet: PieceSet, TRandom: Random<PieceType>, TInputActions: InputActions>(
    piece_set: TPieceSet,
    queue: Queue<PieceType, TRandom>,
    input_actions: TInputActions,
) {
    let mut game = TetrisGame::new(piece_set, queue, input_actions);
    game.start();

    game_loop(
        game,
        60,
        1.0,
        |g| g.game.update(g.fixed_time_step()),
        |g| g.game.render(),
    );
}

struct TetrisGame<TPieceSet: PieceSet, TRandom: Random<PieceType>, TInputActions> {
    board: Board,
    piece_set: TPieceSet,
    active_piece: Option<Piece>,
    queue: Queue<PieceType, TRandom>,
    input_actions: TInputActions,
    hold_piece_type: Option<PieceType>,
    can_hold: bool,

    stdout: RawTerminal<Stdout>,
    stdin: Keys<AsyncReader>,

    drop_timer: f64,
    lines_per_second: i32,
}

impl<TPieceSet: PieceSet, TRandom: Random<PieceType>, TInputActions: InputActions>
    TetrisGame<TPieceSet, TRandom, TInputActions>
{
    fn new(
        piece_set: TPieceSet,
        queue: Queue<PieceType, TRandom>,
        input_actions: TInputActions,
    ) -> Self {
        TetrisGame {
            board: Board::new(),
            piece_set,
            active_piece: None,
            queue,
            input_actions,
            hold_piece_type: None,
            can_hold: true,
            stdout: stdout().into_raw_mode().unwrap(),
            stdin: termion::async_stdin().keys(),
            drop_timer: 0f64,
            lines_per_second: 1,
        }
    }

    fn spawn_piece(&mut self, piece_type: Option<PieceType>) {
        self.active_piece = Some(Piece {
            piece_type: if let Some(t) = piece_type {
                t
            } else {
                self.queue.next()
            },
            rotation: Rotation::Up,
            position: Position::new(4, 19),
        });
    }

    fn start(&mut self) {
        write!(self.stdout, "{}", Hide).unwrap();
        write!(self.stdout, "{}{}", Goto(1, 1), All).unwrap();
        self.stdout.flush().unwrap();
        self.spawn_piece(None);
    }

    fn update(&mut self, delta_time: f64) {
        let actions = self.input_actions.actions(delta_time);
        for action in actions {
            match action {
                Action::MoveLeft => {
                    self.move_active_piece(Position::new(-1, 0));
                }
                Action::MoveRight => {
                    self.move_active_piece(Position::new(1, 0));
                }
                Action::SoftDropStarted => {
                    self.drop_timer *= self.lines_per_second as f64 / 50f64;
                    self.lines_per_second = 50;
                }
                Action::SoftDropStopped => {
                    self.drop_timer *= self.lines_per_second as f64 / 1f64;
                    self.lines_per_second = 1;
                }
                Action::HardDrop => self.hard_drop_active_piece(),
                Action::RotateLeft => {
                    self.rotate_active_piece(Direction::CCW);
                }
                Action::RotateRight => {
                    self.rotate_active_piece(Direction::CW);
                }
                Action::Hold => {
                    if self.can_hold {
                        if let Some(active_piece) = self.active_piece {
                            if let Some(hold_piece_type) = self.hold_piece_type {
                                self.spawn_piece(Some(hold_piece_type));
                            } else {
                                self.spawn_piece(None);
                            }
                            self.hold_piece_type = Some(active_piece.piece_type);
                            self.can_hold = false;
                        }
                    }
                }
                _ => (),
            };
        }

        if let Some(_) = self.active_piece {
            self.drop_timer += delta_time;
            let lines_to_drop = (self.drop_timer * self.lines_per_second as f64).floor() as i32;
            if lines_to_drop > 0 {
                self.drop_timer -= lines_to_drop as f64 / self.lines_per_second as f64;
                if !self.move_active_piece(Position::new(0, -lines_to_drop)) {
                    self.drop_timer = 0f64;
                }
            }
        }
    }

    fn hard_drop_active_piece(&mut self) {
        if let Some(mut active_piece) = self.active_piece {
            active_piece.position = self.board.piece_cast(
                self.piece_set
                    .units(&active_piece.piece_type, &active_piece.rotation),
                &active_piece.position,
                &Position::new(0, -1),
            );
            self.active_piece = Some(active_piece);
            self.lock_active_piece();
        }
    }

    fn lock_active_piece(&mut self) {
        if let Some(active_piece) = self.active_piece {
            self.board.lock_piece(
                self.piece_set
                    .units(&active_piece.piece_type, &active_piece.rotation),
                &active_piece.position,
            );
            self.spawn_piece(None);
            self.can_hold = true;
        }
    }

    fn move_active_piece(&mut self, offset: Position) -> bool {
        if let Some(mut active_piece) = self.active_piece {
            let mut target_position = active_piece.position.clone();
            target_position += offset;
            if !self.board.is_obstructed(
                self.piece_set
                    .units(&active_piece.piece_type, &active_piece.rotation),
                &target_position,
            ) {
                active_piece.position = target_position.clone();
                self.active_piece = Some(active_piece);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn rotate_active_piece(&mut self, direction: Direction) -> bool {
        if let Some(active_piece) = self.active_piece {
            if let Some(piece) = self
                .piece_set
                .rotate_piece(&self.board, &active_piece, direction)
            {
                self.active_piece = Some(piece);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn render(&mut self) {
        let mut composite_board = Array2::<FillType>::from_elem((20, 20), FillType::Empty);
        for row in 0..BOARD_VISIBLE_HEIGHT {
            for col in 0..BOARD_WIDTH {
                let filled = self.board.rows[(BOARD_VISIBLE_HEIGHT - 1) - row][col];
                composite_board[[row, col + 5]] = if filled {
                    FillType::Unit
                } else {
                    FillType::Empty
                }
            }
        }

        if let Some(active_piece) = self.active_piece {
            let ghost_position = self.board.piece_cast(
                self.piece_set
                    .units(&active_piece.piece_type, &active_piece.rotation),
                &active_piece.position,
                &Position::new(0, -1),
            );

            if ghost_position != active_piece.position {
                self.render_piece(
                    active_piece.piece_type,
                    active_piece.rotation,
                    ghost_position + Position::new(5, 0),
                    FillType::Ghost,
                    &mut composite_board,
                );
            }

            if let Some(hold_piece_type) = self.hold_piece_type {
                self.render_piece(
                    hold_piece_type,
                    Rotation::Up,
                    Position::new(1, 16),
                    FillType::Unit,
                    &mut composite_board,
                );
            }

            let next_pieces = self.queue.next_items().to_vec();
            for i in 0..next_pieces.len() {
                let next_piece_type = *next_pieces.get(i).unwrap();
                self.render_piece(
                    next_piece_type,
                    Rotation::Up,
                    Position::new(17, (16 - 3 * i) as i32),
                    FillType::Unit,
                    &mut composite_board,
                );
            }

            self.render_piece(
                active_piece.piece_type,
                active_piece.rotation,
                active_piece.position + Position::new(5, 0),
                FillType::Unit,
                &mut composite_board,
            );
        }

        let mut render = String::new();
        for row in 0..20 {
            for col in 0..20 {
                match composite_board[[row, col]] {
                    FillType::Unit => render.push_str("[]"),
                    FillType::Empty => {
                        if (5..15).contains(&col) {
                            render.push_str("..")
                        } else {
                            render.push_str("  ")
                        }
                    }
                    FillType::Ghost => render.push_str("=="),
                }
            }
            render.push_str("\r\n");
        }
        render.replace_range(2..6, "HOLD");
        render.replace_range(34..38, "NEXT");

        write!(self.stdout, "{}{}", Goto(1, 1), render).unwrap();
        self.stdout.flush().unwrap();
    }

    fn render_piece(
        &mut self,
        piece_type: PieceType,
        rotation: Rotation,
        position: Position,
        fill_type: FillType,
        composite_board: &mut Array2<FillType>,
    ) {
        let units = self.piece_set.units(&piece_type, &rotation);
        for unit in units {
            let x = position.x + unit.x;
            let y = (BOARD_VISIBLE_HEIGHT - 1) as i32 - (position.y + unit.y);
            if let [height, width] = composite_board.shape() {
                if (0..(*width as i32)).contains(&x) && (0..(*height as i32)).contains(&y) {
                    composite_board[[y as usize, x as usize]] = fill_type;
                }
            }
        }
    }
}
