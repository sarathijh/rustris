use super::{
    board::Board,
    input::{Action, InputActions},
    piece::{Direction, Piece, PieceSet, PieceType, Rotation},
    position::Position,
    queue::Queue,
    random::Random,
    renderer::{RenderState, Renderer},
};

pub struct TetrisGame<
    TPieceSet: PieceSet,
    TRandom: Random<PieceType>,
    TInputActions,
    TRenderer: Renderer<TPieceSet>,
> {
    board: Board,
    piece_set: TPieceSet,
    active_piece: Option<Piece>,
    queue: Queue<PieceType, TRandom>,
    input_actions: TInputActions,
    hold_piece_type: Option<PieceType>,
    can_hold: bool,
    drop_timer: f64,
    lines_per_second: i32,
    renderer: TRenderer,
    paused: bool,
}

impl<
        TPieceSet: PieceSet,
        TRandom: Random<PieceType>,
        TInputActions: InputActions,
        TRenderer: Renderer<TPieceSet>,
    > TetrisGame<TPieceSet, TRandom, TInputActions, TRenderer>
{
    pub fn new(
        piece_set: TPieceSet,
        queue: Queue<PieceType, TRandom>,
        input_actions: TInputActions,
        renderer: TRenderer,
    ) -> Self {
        TetrisGame {
            board: Board::new(),
            piece_set,
            active_piece: None,
            queue,
            input_actions,
            hold_piece_type: None,
            can_hold: true,
            drop_timer: 0f64,
            lines_per_second: 1,
            renderer,
            paused: false,
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

    pub fn init(&mut self) {
        self.renderer.init();
        self.spawn_piece(None);
    }

    pub fn update(&mut self, delta_time: f64) {
        let actions = self.input_actions.actions(delta_time);

        if actions.contains(&Action::Pause) {
            self.paused = !self.paused;
        }

        if self.paused {
            return;
        }

        for action in actions {
            match action {
                Action::Pause => {
                    // Already handled above
                }
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

    pub fn render(&mut self) {
        let ghost_piece_position = if let Some(active_piece) = self.active_piece {
            Some(
                self.board.piece_cast(
                    self.piece_set
                        .units(&active_piece.piece_type, &active_piece.rotation),
                    &active_piece.position,
                    &Position::new(0, -1),
                ),
            )
        } else {
            None
        };

        let render_state = RenderState::new(
            self.board.rows.to_vec(),
            &self.piece_set,
            self.active_piece,
            ghost_piece_position,
            self.hold_piece_type,
            self.queue.next_items().to_vec(),
            self.paused,
        );

        self.renderer.render(render_state);
    }
}
