use super::position::Position;

pub struct Board {
    pub rows: Vec<[bool; 10]>,
}

impl Board {
    pub fn new() -> Self {
        let mut rows = Vec::new();

        for _ in 0..40 {
            rows.push([
                false, false, false, false, false, false, false, false, false, false,
            ])
        }

        Board { rows }
    }

    pub fn lock_piece(&mut self, units: [Position; 4], offset: &Position) {
        for unit in units {
            let position = unit + *offset;
            self.rows[position.y as usize][position.x as usize] = true;
        }
    }

    pub fn piece_cast(
        &self,
        units: [Position; 4],
        offset: &Position,
        direction: &Position,
    ) -> Position {
        let mut position = *offset;
        loop {
            position += *direction;
            if self.is_obstructed(units, &position) {
                position -= *direction;
                break;
            }
        }
        position
    }

    pub fn is_obstructed(&self, units: [Position; 4], offset: &Position) -> bool {
        for position in units {
            let cell = position + offset.clone();
            if cell.x < 0 || cell.x >= 10 || cell.y < 0 || cell.y >= 40 {
                return true;
            }
            if self.rows[cell.y as usize][cell.x as usize] {
                return true;
            }
        }
        return false;
    }
}
