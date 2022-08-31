use super::{
    board::Board,
    input::{Action, InputActions},
    piece::{Direction, Piece, PieceSet, PieceType, Rotation},
    position::Position,
    queue::Queue,
    random::Random,
    renderer::{RenderState, Renderer},
};

struct HoldFeature {
    can_hold: bool,
    hold_piece_type: Option<PieceType>,
}

impl HoldFeature {
    fn new() -> Self {
        Self {
            can_hold: true,
            hold_piece_type: None,
        }
    }

    fn reset(&mut self) {
        self.can_hold = true;
    }

    fn hold(&mut self, piece_type: PieceType) -> Option<Option<PieceType>> {
        if self.can_hold {
            let spawn_piece_type = if let Some(hold_piece_type) = self.hold_piece_type {
                Some(Some(hold_piece_type))
            } else {
                Some(None)
            };
            self.hold_piece_type = Some(piece_type);
            self.can_hold = false;
            spawn_piece_type
        } else {
            None
        }
    }
}

struct GravityFeature {
    drop_timer: f64,
    lines_per_second: i32,
}

impl GravityFeature {
    fn new(lines_per_second: i32) -> Self {
        Self {
            drop_timer: 0f64,
            lines_per_second,
        }
    }

    fn set_lines_per_second(&mut self, lines_per_second: i32) {
        // We need to update our drop timer so the piece finishes dropping at the previous speed,
        // otherwise the piece will drop too much if the lines_per_second is increased
        self.drop_timer *= self.lines_per_second as f64 / lines_per_second as f64;
        self.lines_per_second = lines_per_second;
    }

    fn update_drop(&mut self, delta_time: f64) -> i32 {
        self.drop_timer += delta_time;
        let lines_to_drop = (self.drop_timer * self.lines_per_second as f64).floor() as i32;
        if lines_to_drop > 0 {
            self.drop_timer -= lines_to_drop as f64 / self.lines_per_second as f64;
        }
        lines_to_drop
    }
}

pub struct TetrisGame<
    TPieceSet: PieceSet,
    TRandom: Random<PieceType>,
    TInputActions,
    TRenderer: Renderer<TPieceSet>,
> {
    board: Board,
    piece_set: TPieceSet,
    active_piece: Option<Piece>,
    ghost_piece_position: Option<Position>,
    queue: Queue<PieceType, TRandom>,
    input_actions: TInputActions,
    hold_feature: HoldFeature,
    gravity_feature: GravityFeature,
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
            ghost_piece_position: None,
            queue,
            input_actions,
            hold_feature: HoldFeature::new(),
            gravity_feature: GravityFeature::new(1),
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
        self.update_ghost_piece_position();
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
                    self.move_active_piece(Position::left());
                }
                Action::MoveRight => {
                    self.move_active_piece(Position::right());
                }
                Action::SoftDropStarted => self.gravity_feature.set_lines_per_second(50),
                Action::SoftDropStopped => self.gravity_feature.set_lines_per_second(1),
                Action::HardDrop => self.hard_drop_active_piece(),
                Action::RotateLeft => {
                    self.rotate_active_piece(Direction::CCW);
                }
                Action::RotateRight => {
                    self.rotate_active_piece(Direction::CW);
                }
                Action::Hold => {
                    // We can only hold if we have an active piece spawned
                    if let Some(active_piece) = self.active_piece {
                        if let Some(piece_to_spawn) =
                            self.hold_feature.hold(active_piece.piece_type)
                        {
                            self.spawn_piece(piece_to_spawn);
                        }
                    }
                }
            };
        }

        if let Some(_) = self.active_piece {
            let lines_to_drop = self.gravity_feature.update_drop(delta_time);
            if lines_to_drop > 0 {
                self.move_active_piece(lines_to_drop * Position::down());
            }
        }
    }

    fn hard_drop_active_piece(&mut self) {
        if let Some(mut active_piece) = self.active_piece {
            active_piece.position =
                self.board
                    .piece_cast(&self.piece_set, &active_piece, &Position::down());
            self.active_piece = Some(active_piece);
            self.update_ghost_piece_position();
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
            self.hold_feature.reset();
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
                self.update_ghost_piece_position();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn update_ghost_piece_position(&mut self) {
        self.ghost_piece_position = if let Some(active_piece) = self.active_piece {
            Some(
                self.board
                    .piece_cast(&self.piece_set, &active_piece, &Position::down()),
            )
        } else {
            None
        };
    }

    fn rotate_active_piece(&mut self, direction: Direction) -> bool {
        if let Some(active_piece) = self.active_piece {
            if let Some(piece) = self
                .piece_set
                .rotate_piece(&self.board, &active_piece, direction)
            {
                self.active_piece = Some(piece);
                self.update_ghost_piece_position();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn render(&mut self) {
        let render_state = RenderState::new(
            self.board.rows.to_vec(),
            &self.piece_set,
            self.active_piece,
            self.ghost_piece_position,
            self.hold_feature.hold_piece_type,
            self.queue.next_items().to_vec(),
            self.paused,
        );

        self.renderer.render(render_state);
    }
}
