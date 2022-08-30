use std::io::{stdout, Stdout};

use ndarray::Array2;
use std::io::Write;
use termion::{
    clear::All,
    cursor::{Goto, Hide},
    raw::{IntoRawMode, RawTerminal},
};

use super::{
    piece::{PieceSet, PieceType, Rotation},
    renderer::Renderer,
};
use super::{position::Position, renderer::RenderState};

const BOARD_WIDTH: usize = 10;
const BOARD_VISIBLE_HEIGHT: usize = 20;

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
        fill_type: FillType,
        render_ir: &mut Array2<FillType>,
    ) {
        let units = piece_set.units(&piece_type, &rotation);
        for unit in units {
            let x = position.x + unit.x;
            let y = (BOARD_VISIBLE_HEIGHT - 1) as i32 - (position.y + unit.y);
            if let [height, width] = render_ir.shape() {
                if (0..(*width as i32)).contains(&x) && (0..(*height as i32)).contains(&y) {
                    render_ir[[y as usize, x as usize]] = fill_type;
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
        // An intermediate representation for the final render
        // This makes it easy to composite all the components before converting it to a string
        let mut render_ir = Array2::<FillType>::from_elem((20, 20), FillType::Empty);

        // Render the board state into the intermediate representation
        for row in 0..BOARD_VISIBLE_HEIGHT {
            for col in 0..BOARD_WIDTH {
                let filled = state.board_state[(BOARD_VISIBLE_HEIGHT - 1) - row][col];
                render_ir[[row, col + 5]] = if filled {
                    FillType::Filled
                } else {
                    FillType::Field
                }
            }
        }

        // Render the queue of next pieces into the intermediate representation
        for i in 0..state.next_piece_types.len() {
            let next_piece_type = *state.next_piece_types.get(i).unwrap();
            self.render_piece(
                state.piece_set,
                next_piece_type,
                Rotation::Up,
                Position::new(17, (16 - 3 * i) as i32),
                FillType::Filled,
                &mut render_ir,
            );
        }

        // Render the hold piece into the intermediate representation
        if let Some(hold_piece_type) = state.hold_piece_type {
            self.render_piece(
                state.piece_set,
                hold_piece_type,
                Rotation::Up,
                Position::new(1, 16),
                FillType::Filled,
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
                    ghost_piece_position + Position::new(5, 0),
                    FillType::Ghost,
                    &mut render_ir,
                );
            }

            // Render the active piece into the intermediate representation
            self.render_piece(
                state.piece_set,
                active_piece.piece_type,
                active_piece.rotation,
                active_piece.position + Position::new(5, 0),
                FillType::Filled,
                &mut render_ir,
            );
        }

        // Create a string builder for the final render
        let mut render = String::new();

        // Convert the intermediate representation into the final string render
        for row in 0..20 {
            for col in 0..20 {
                match render_ir[[row, col]] {
                    FillType::Filled => render.push_str("[]"),
                    FillType::Empty => render.push_str("  "),
                    FillType::Field => render.push_str(".."),
                    FillType::Ghost => render.push_str("++"),
                }
            }
            render.push_str("\r\n");
        }

        render.replace_range(2..6, "HOLD");
        render.replace_range(34..38, "NEXT");

        // Output the string render to stdout
        write!(self.stdout, "{}{}", Goto(1, 1), render).unwrap();
        self.stdout.flush().unwrap();
    }
}
