use crate::game::Message;

use super::{
    piece::{Piece, PieceSet, PieceType},
    position::Position,
};

pub type BoardState = Vec<[bool; 10]>;

pub struct RenderState<'a, TPieceSet: PieceSet> {
    pub board_state: BoardState,
    pub piece_set: &'a TPieceSet,
    pub active_piece: Option<Piece>,
    pub ghost_piece_position: Option<Position>,
    pub hold_piece_type: Option<PieceType>,
    pub next_piece_types: Vec<PieceType>,
    pub paused: bool,
    pub messages: Vec<Message>,
}

impl<'a, TPieceSet: PieceSet> RenderState<'a, TPieceSet> {
    pub fn new(
        board_state: BoardState,
        piece_set: &'a TPieceSet,
        active_piece: Option<Piece>,
        ghost_piece_position: Option<Position>,
        hold_piece_type: Option<PieceType>,
        next_piece_types: Vec<PieceType>,
        paused: bool,
        messages: Vec<Message>,
    ) -> Self {
        Self {
            board_state,
            piece_set,
            active_piece,
            ghost_piece_position,
            hold_piece_type,
            next_piece_types,
            paused,
            messages,
        }
    }
}

pub trait Renderer<TPieceSet: PieceSet> {
    fn init(&mut self);
    fn render(&mut self, state: RenderState<TPieceSet>, delta_time: f64);
}
