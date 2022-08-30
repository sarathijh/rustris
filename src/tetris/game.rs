use std::io::{stdout, Stdout};

use game_loop::game_loop;
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
    piece::{Direction, Piece, PieceSet, PieceType, Rotation},
    position::Position,
    queue::Queue,
    random::Random,
};

const BOARD_WIDTH: usize = 10;
const BOARD_VISIBLE_HEIGHT: usize = 20;

pub fn tetris<TPieceSet: PieceSet, TRandom: Random<PieceType>>(
    piece_set: TPieceSet,
    queue: Queue<PieceType, TRandom>,
) {
    let mut game = TetrisGame::new(piece_set, queue);
    game.start();

    game_loop(
        game,
        60,
        1.0,
        |g| g.game.update(g.fixed_time_step()),
        |g| g.game.render(),
    );
}

struct TetrisGame<TPieceSet: PieceSet, TRandom: Random<PieceType>> {
    board: Board,
    piece_set: TPieceSet,
    active_piece: Option<Piece>,
    queue: Queue<PieceType, TRandom>,

    stdout: RawTerminal<Stdout>,
    stdin: Keys<AsyncReader>,

    drop_timer: f64,
    lines_per_second: i32,
}

impl<TPieceSet: PieceSet, TRandom: Random<PieceType>> TetrisGame<TPieceSet, TRandom> {
    fn new(piece_set: TPieceSet, queue: Queue<PieceType, TRandom>) -> Self {
        TetrisGame {
            board: Board::new(),
            piece_set,
            active_piece: None,
            queue,
            stdout: stdout().into_raw_mode().unwrap(),
            stdin: termion::async_stdin().keys(),
            drop_timer: 0f64,
            lines_per_second: 1,
        }
    }

    fn spawn_piece(&mut self) {
        self.active_piece = Some(Piece {
            piece_type: self.queue.next(),
            rotation: Rotation::Up,
            position: Position::new(4, 19),
        });
    }

    fn start(&mut self) {
        write!(self.stdout, "{}", Hide).unwrap();
        write!(self.stdout, "{}{}", Goto(1, 1), All).unwrap();
        self.stdout.flush().unwrap();
        self.spawn_piece();
    }

    fn update(&mut self, delta_time: f64) {
        let input = self.stdin.next();
        if let Some(Ok(key)) = input {
            let _ = match key {
                Key::Left => self.move_active_piece(Position::new(-1, 0)),
                Key::Right => self.move_active_piece(Position::new(1, 0)),
                Key::Down => self.move_active_piece(Position::new(0, -1)),
                Key::Up => {
                    self.hard_drop_active_piece();
                    false
                }
                Key::Char('z') => self.rotate_active_piece(Direction::CCW),
                Key::Char('x') => self.rotate_active_piece(Direction::CW),
                _ => false,
            };
        }

        if let Some(_) = self.active_piece {
            self.drop_timer += delta_time;
            let lines_to_drop = (self.drop_timer * self.lines_per_second as f64).floor() as i32;
            if lines_to_drop > 0 {
                self.drop_timer -= lines_to_drop as f64 / self.lines_per_second as f64;
                if !self.move_active_piece(Position::new(0, -lines_to_drop)) {
                    self.lock_active_piece();
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
            self.spawn_piece();
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
        let mut composite_board = Vec::<[bool; 10]>::new();
        for row in 0..BOARD_VISIBLE_HEIGHT {
            composite_board.push(self.board.rows[(BOARD_VISIBLE_HEIGHT - 1) - row]);
        }

        self.render_piece(&mut composite_board);

        let mut render = String::new();
        for row in 0..BOARD_VISIBLE_HEIGHT {
            for col in 0..BOARD_WIDTH {
                if composite_board[row][col] {
                    render.push_str("[]");
                } else {
                    render.push_str("..");
                }
            }
            render.push_str("\r\n");
        }

        write!(self.stdout, "{}{}", Goto(1, 1), render).unwrap();
        self.stdout.flush().unwrap();
    }

    fn render_piece(&mut self, composite_board: &mut Vec<[bool; 10]>) {
        if let Some(active_piece) = self.active_piece {
            let units = self
                .piece_set
                .units(&active_piece.piece_type, &active_piece.rotation);
            for unit in units {
                let x = active_piece.position.x + unit.x;
                let y = (BOARD_VISIBLE_HEIGHT - 1) as i32 - (active_piece.position.y + unit.y);
                if y >= 0 && y < BOARD_VISIBLE_HEIGHT as i32 && x >= 0 && x < BOARD_WIDTH as i32 {
                    composite_board[y as usize][x as usize] = true;
                }
            }
        }
    }
}
