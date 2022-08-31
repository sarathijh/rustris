use std::io::{stdout, Stdout};

use ndarray::Array2;
use std::io::Write;
use termion::{
    clear::All,
    cursor::{Goto, Hide},
    raw::{IntoRawMode, RawTerminal},
};

use tetris_core::{
    piece::{PieceSet, PieceType, Rotation},
    renderer::Renderer,
};
use tetris_core::{position::Position, renderer::RenderState};

const CELL_WIDTH: usize = 2;
const CELL_HEIGHT: usize = 1;

const LEFT_CONTENT_WIDTH: usize = 5;
const RIGHT_CONTENT_WIDTH: usize = 5;

const LEFT_BORDER_WIDTH: usize = 1;
const RIGHT_BORDER_WIDTH: usize = 1;
const TOP_BORDER_HEIGHT: usize = 0;
const BOTTOM_BORDER_HEIGHT: usize = 2;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

#[derive(Clone, Copy)]
enum FillType {
    Empty,
    Field,
    Filled,
    Ghost,
}

pub struct TermionRenderer {
    stdout: RawTerminal<Stdout>,
}

impl TermionRenderer {
    pub fn new() -> Self {
        Self {
            stdout: stdout().into_raw_mode().unwrap(),
        }
    }
}

impl TermionRenderer {
    fn render_piece<TPieceSet: PieceSet>(
        &mut self,
        piece_set: &TPieceSet,
        piece_type: PieceType,
        rotation: Rotation,
        position: Position,
        fill_type: &str,
        render_ir: &mut Array2<char>,
    ) {
        let units = piece_set.units(&piece_type, &rotation);
        for unit in units {
            let x = position.x + unit.x * CELL_WIDTH as i32;
            let y = position.y - unit.y * CELL_HEIGHT as i32;
            if let [height, width] = render_ir.shape() {
                if (0..(*width as i32)).contains(&x) && (0..(*height as i32)).contains(&y) {
                    for i in 0..fill_type.len() {
                        render_ir[[y as usize, x as usize + i]] = fill_type.chars().nth(i).unwrap();
                    }
                }
            }
        }
    }
}

impl<TPieceSet: PieceSet> Renderer<TPieceSet> for TermionRenderer {
    fn init(&mut self) {
        write!(self.stdout, "{}", Hide).unwrap();
        write!(self.stdout, "{}{}", Goto(1, 1), All).unwrap();
        self.stdout.flush().unwrap();
    }

    fn render(&mut self, state: RenderState<TPieceSet>) {
        let render_width = (LEFT_CONTENT_WIDTH
            + LEFT_BORDER_WIDTH
            + BOARD_WIDTH
            + RIGHT_BORDER_WIDTH
            + RIGHT_CONTENT_WIDTH)
            * CELL_WIDTH;

        let render_height = (TOP_BORDER_HEIGHT + BOARD_HEIGHT + BOTTOM_BORDER_HEIGHT) * CELL_HEIGHT;

        let board_start_x = (LEFT_CONTENT_WIDTH + LEFT_BORDER_WIDTH) * CELL_WIDTH;
        let right_content_start_x =
            board_start_x + (BOARD_WIDTH + RIGHT_BORDER_WIDTH + 1) * CELL_WIDTH;

        let board_start_y = (TOP_BORDER_HEIGHT) * CELL_HEIGHT;

        let to_render_position = |position: &Position| -> Position {
            Position::new(
                board_start_x as i32 + position.x * CELL_WIDTH as i32,
                board_start_y as i32 + (BOARD_HEIGHT as i32 - 1) - position.y * CELL_HEIGHT as i32,
            )
        };

        // An intermediate representation for the final render
        // This makes it easy to composite all the components before converting it to a string
        let mut render_ir = Array2::<char>::from_elem((render_height, render_width), ' ');

        // Render the board state into the intermediate representation
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                let filled = state.board_state[(BOARD_HEIGHT - 1) - row][col];
                if filled {
                    render_ir[[row, board_start_x + col * CELL_WIDTH]] = '[';
                    render_ir[[row, board_start_x + col * CELL_WIDTH + 1]] = ']';
                } else {
                    render_ir[[row, board_start_x + col * CELL_WIDTH]] = ' ';
                    render_ir[[row, board_start_x + col * CELL_WIDTH + 1]] = '.';
                }
            }
        }

        for row in 0..(BOARD_HEIGHT + 1) {
            render_ir[[row, LEFT_CONTENT_WIDTH * CELL_WIDTH]] = '<';
            render_ir[[row, LEFT_CONTENT_WIDTH * CELL_WIDTH + 1]] = '!';

            render_ir[[
                row,
                (LEFT_CONTENT_WIDTH + LEFT_BORDER_WIDTH + BOARD_WIDTH) * CELL_WIDTH,
            ]] = '!';
            render_ir[[
                row,
                (LEFT_CONTENT_WIDTH + LEFT_BORDER_WIDTH + BOARD_WIDTH) * CELL_WIDTH + 1,
            ]] = '>';
        }

        for col in 0..BOARD_WIDTH {
            render_ir[[
                BOARD_HEIGHT,
                (LEFT_CONTENT_WIDTH + LEFT_BORDER_WIDTH + col) * CELL_WIDTH,
            ]] = '=';
            render_ir[[
                BOARD_HEIGHT,
                (LEFT_CONTENT_WIDTH + LEFT_BORDER_WIDTH + col) * CELL_WIDTH + 1,
            ]] = '=';

            render_ir[[
                BOARD_HEIGHT + 1,
                (LEFT_CONTENT_WIDTH + LEFT_BORDER_WIDTH + col) * CELL_WIDTH,
            ]] = '\\';
            render_ir[[
                BOARD_HEIGHT + 1,
                (LEFT_CONTENT_WIDTH + LEFT_BORDER_WIDTH + col) * CELL_WIDTH + 1,
            ]] = '/';
        }

        fn piece_type_center_offset_x(piece_type: PieceType) -> usize {
            match piece_type {
                PieceType::I | PieceType::O => 0,
                PieceType::T | PieceType::J | PieceType::L | PieceType::Z | PieceType::S => 1,
            }
        }

        // Render the queue of next pieces into the intermediate representation
        for i in 0..state.next_piece_types.len() {
            let next_piece_type = *state.next_piece_types.get(i).unwrap();
            self.render_piece(
                state.piece_set,
                next_piece_type,
                Rotation::Up,
                Position::new(
                    (right_content_start_x
                        + CELL_WIDTH
                        + piece_type_center_offset_x(next_piece_type)) as i32,
                    (3 + 3 * i) as i32,
                ),
                "[]",
                &mut render_ir,
            );
        }

        // Render the hold piece into the intermediate representation
        if let Some(hold_piece_type) = state.hold_piece_type {
            self.render_piece(
                state.piece_set,
                hold_piece_type,
                Rotation::Up,
                Position::new(2 + piece_type_center_offset_x(hold_piece_type) as i32, 3),
                "[]",
                &mut render_ir,
            );
        }

        if let Some(active_piece) = state.active_piece {
            // Render the ghost piece into the intermediate representation
            if let Some(ghost_piece_position) = state.ghost_piece_position {
                self.render_piece(
                    state.piece_set,
                    active_piece.piece_type,
                    active_piece.rotation,
                    to_render_position(&ghost_piece_position),
                    "++",
                    &mut render_ir,
                );
            }

            // Render the active piece into the intermediate representation
            self.render_piece(
                state.piece_set,
                active_piece.piece_type,
                active_piece.rotation,
                to_render_position(&active_piece.position),
                "[]",
                &mut render_ir,
            );
        }

        render_ir[[0, 2]] = 'H';
        render_ir[[0, 3]] = 'O';
        render_ir[[0, 4]] = 'L';
        render_ir[[0, 5]] = 'D';

        render_ir[[0, right_content_start_x + 2]] = 'N';
        render_ir[[0, right_content_start_x + 3]] = 'E';
        render_ir[[0, right_content_start_x + 4]] = 'X';
        render_ir[[0, right_content_start_x + 5]] = 'T';

        // Create a string builder for the final render
        let mut render = String::new();

        // Convert the intermediate representation into the final string render
        for row in 0..render_height {
            for col in 0..render_width {
                render.push(render_ir[[row, col]]);
            }
            render.push_str("\r\n");
        }

        // Output the string render to stdout
        write!(self.stdout, "{}{}", Goto(1, 1), render).unwrap();
        self.stdout.flush().unwrap();
    }
}
