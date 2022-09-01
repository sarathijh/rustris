use crate::piece::{Piece, PieceSet};

use super::position::Position;

/// A Board is a collection of rows, each 10 columns wide
/// The standard board height is 40 rows (20 of which aren't visible above the playfield)
pub struct Board {
    pub rows: Vec<[bool; 10]>,
}

impl Board {
    /// Creates an empty board
    pub fn new() -> Self {
        let mut rows = Vec::new();

        for _ in 0..40 {
            rows.push([
                false, false, false, false, false, false, false, false, false, false,
            ])
        }

        Board { rows }
    }

    /// Adds the piece's units permanently to the board
    pub fn lock_piece(&mut self, units: [Position; 4], offset: Position) -> usize {
        for unit in units {
            let position = unit + offset;
            self.rows[position.y as usize][position.x as usize] = true;
        }

        // Check if lines need to be cleared after the piece is locked
        self.clear_lines()
    }

    /// Removes all filled rows
    /// When a row is removed, all rows above it are moved down
    fn clear_lines(&mut self) -> usize {
        let mut lines_cleared: usize = 0;
        for row in (0..40).rev() {
            if self.rows[row].iter().all(|it| *it) {
                for i in row..39 {
                    let next_row = self.rows[i + 1];
                    self.rows[i].copy_from_slice(&next_row);
                }
                self.rows[39] = [
                    false, false, false, false, false, false, false, false, false, false,
                ];
                lines_cleared += 1;
            }
        }
        lines_cleared
    }

    pub fn is_all_clear(&self) -> bool {
        self.rows.iter().all(|row| row.iter().all(|cell| !cell))
    }

    /// Determines how far a piece can move in the given direction before it is obstructed
    pub fn piece_cast(
        &self,
        piece_set: &dyn PieceSet,
        piece: Piece,
        direction: Position,
    ) -> Position {
        let units = piece_set.units(&piece.piece_type, &piece.rotation);
        let mut position = piece.position;
        while !self.is_obstructed(units, position + direction) {
            position += direction;
        }
        position
    }

    /// Returns true if any unit of the piece is occupying a filled space on the board
    /// or is outside the bounds of the board
    pub fn is_obstructed(&self, units: [Position; 4], position: Position) -> bool {
        return units
            .iter()
            .any(|unit_offset| self.is_filled(position + *unit_offset));
    }

    pub fn is_filled(&self, position: Position) -> bool {
        if !(0..10).contains(&position.x) || !(0..40).contains(&position.y) {
            return true;
        }
        return self.rows[position.y as usize][position.x as usize];
    }
}
