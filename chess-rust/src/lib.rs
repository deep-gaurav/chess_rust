use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Board {
    pub positions: HashMap<Position, Option<Piece>>,
    pub turn: Color,
    pub width: u32,
    pub team_mode: TeamMode,
    pub game_type: GameType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum GameType {
    TwoPlayer,
    ThreePlayer,
    FourPlayer,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TeamMode {
    Team,
    Solo,
}

pub type Position = (i32, i32);

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, Clone)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub promotion: Option<Piece>,
}

impl Board {
    pub const FIRST_COLOR: Color = Color::White;

    pub fn init_2p() -> Self {
        let mut positions = HashMap::new();
        for i in 0..8 {
            for j in 0..8 {
                let piece;
                if i == 0 || i == 7 {
                    let color = {
                        if i == 0 {
                            Color::White
                        } else {
                            Color::Black
                        }
                    };
                    match j {
                        0 | 7 => piece = Some(Piece::Rook(color)),
                        1 | 6 => piece = Some(Piece::Knight(color)),
                        2 | 5 => piece = Some(Piece::Bishop(color)),
                        3 => piece = Some(Piece::Queen(color)),
                        4 => piece = Some(Piece::King(color)),
                        _ => piece = None,
                    }
                } else if i == 1 {
                    piece = Some(Piece::Pawn(Color::White));
                } else if 7 - i == 1 {
                    piece = Some(Piece::Pawn(Color::Black));
                } else {
                    piece = None;
                }
                positions.insert((i, j), piece);
            }
        }
        Board {
            positions,
            turn: Color::White,
            width: 8,
            team_mode: TeamMode::Solo,
            game_type: GameType::TwoPlayer,
        }
    }

    pub fn init_4p(team_mode: TeamMode) -> Self {
        let mut positions = HashMap::new();
        for i in 0..14 {
            for j in 0..14 {
                let piece;
                if (i < 3 && j < 3) || (i > 10 && j < 3) || (i < 3 && j > 10) || (i > 10 && j > 10)
                {
                    continue;
                } else if i == 0 || i == 13 {
                    let color = {
                        if i == 0 {
                            Color::White
                        } else {
                            Color::Black
                        }
                    };
                    match j {
                        3 | 10 => piece = Some(Piece::Rook(color)),
                        4 | 9 => piece = Some(Piece::Knight(color)),
                        5 | 8 => piece = Some(Piece::Bishop(color)),
                        6 => piece = Some(Piece::Queen(color)),
                        7 => piece = Some(Piece::King(color)),
                        _ => piece = None,
                    }
                } else if i == 1 {
                    piece = Some(Piece::Pawn(Color::White));
                } else if 13 - i == 1 {
                    piece = Some(Piece::Pawn(Color::Black));
                } else {
                    piece = None;
                }
                positions.insert((i, j), piece);
            }
        }
        for i in 0..14 {
            for j in 0..14 {
                let piece;
                if (i < 3 && j < 3) || (i > 10 && j < 3) || (i < 3 && j > 10) || (i > 10 && j > 10)
                {
                    continue;
                } else if i == 0 || i == 13 {
                    let color = {
                        if i == 0 {
                            Color::Blue
                        } else {
                            Color::Red
                        }
                    };
                    match j {
                        3 | 10 => piece = Some(Piece::Rook(color)),
                        4 | 9 => piece = Some(Piece::Knight(color)),
                        5 | 8 => piece = Some(Piece::Bishop(color)),
                        6 => piece = Some(Piece::Queen(color)),
                        7 => piece = Some(Piece::King(color)),
                        _ => piece = None,
                    }
                } else if i == 1 {
                    piece = Some(Piece::Pawn(Color::Blue));
                } else if 13 - i == 1 {
                    piece = Some(Piece::Pawn(Color::Red));
                } else {
                    piece = None;
                }
                if piece != None {
                    positions.insert((j, i), piece);
                }
            }
        }
        Board {
            positions,
            turn: Color::White,
            width: 14,
            team_mode: team_mode,
            game_type: GameType::FourPlayer,
        }
    }

    pub fn get_position_color(position: &Position) -> Color {
        if (position.0 + position.1) % 2 == 1 {
            Color::White
        } else {
            Color::Black
        }
    }
}

impl Board {
    pub fn is_check(&self, color: Color) -> bool {
        let king_position = self.positions.iter().find_map(|(k, v)| {
            if v == &Some(Piece::King(color)) {
                Some(k)
            } else {
                None
            }
        });
        if let Some(king_position) = king_position {
            let opponents = self
                .positions
                .iter()
                .filter_map(|(k, p)| {
                    if let Some(piece) = p {
                        Some((k, piece))
                    } else {
                        None
                    }
                })
                .filter(|(k, p)| p.get_color().is_opponent(&color, &self.team_mode));
            let mut cap_moves = vec![];
            for opp in opponents {
                cap_moves.append(&mut opp.1.get_hit_moves(self, opp.0));
            }
            cap_moves.iter().any(|mov| mov.to == *king_position)
        } else {
            true
        }
    }

    pub fn is_checkmate(&self, color: Color) -> bool {
        if self.is_check(color) && self.get_all_legal_moves(color).is_empty() {
            true
        } else {
            false
        }
    }

    pub fn is_position_capturable(&self, position: &Position, color: &Color) -> bool {
        if let Some(piece) = (|| Some(self.positions.get(position)?.clone()?))() {
            piece.get_color().is_opponent(color, &self.team_mode)
        } else {
            false
        }
    }

    pub fn is_position_occupied(&self, position: &Position) -> bool {
        if let Some(position) = self.positions.get(position) {
            if let Some(_) = position {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn is_position_moveable(&self, position: &Position, color: &Color) -> bool {
        if let Some(position) = self.positions.get(position) {
            if let Some(piece) = position {
                if piece.get_color().is_opponent(color, &self.team_mode) {
                    true
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn is_game_over(&self) -> bool {
        match self.game_type {
            GameType::TwoPlayer => {
                self.is_checkmate(Color::Black) || self.is_checkmate(Color::White)
            }
            GameType::ThreePlayer => {
                let mut mates = 0;
                for color in [Color::White, Color::Black, Color::Red].iter() {
                    if self.is_checkmate(color.clone()) {
                        mates += 1;
                    }
                }
                if mates >= 2 {
                    true
                } else {
                    false
                }
            }
            GameType::FourPlayer => {
                let mut mates = 0;
                for color in [Color::White, Color::Black, Color::Red, Color::Blue].iter() {
                    if self.is_checkmate(color.clone()) {
                        mates += 1;
                    }
                }
                match self.team_mode {
                    TeamMode::Team => {
                        if mates >= 1 {
                            true
                        } else {
                            false
                        }
                    }
                    TeamMode::Solo => {
                        if mates >= 3 {
                            true
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }

    pub fn change_turn(&mut self) {
        self.turn = match self.turn {
            Color::White => match self.game_type {
                GameType::TwoPlayer => Color::Black,
                GameType::FourPlayer | GameType::ThreePlayer => Color::Red,
            },
            Color::Black => match self.game_type {
                GameType::TwoPlayer | GameType::ThreePlayer => Color::White,
                GameType::FourPlayer => Color::Blue,
            },
            Color::Red => Color::Black,
            Color::Blue => Color::White,
        }
    }

    pub fn apply_move(&mut self, mov: &Move) {
        let piece = self.positions.insert(mov.from, None);
        if let Some(promotion) = mov.promotion {
            self.positions.insert(mov.to, Some(promotion));
        } else if let Some(piece) = piece {
            self.positions.insert(mov.to, piece);
        }
    }

    pub fn is_move_legal(&self, mov: &Move) -> bool {
        if self.positions.get(&mov.to).is_none() {
            return false;
        }
        if let Some(piece) = self.positions.get(&mov.from) {
            if let Some(piece) = piece {
                let mut tmp_board = self.clone();
                tmp_board.apply_move(mov);
                !tmp_board.is_check(piece.get_color())
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn is_move_correct_turn(&self, mov: &Move) -> bool {
        if self.positions.get(&mov.to).is_none() {
            return false;
        }
        if let Some(piece) = self.positions.get(&mov.from) {
            if let Some(piece) = piece {
                if piece.get_color() == self.turn {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_all_legal_moves(&self, color: Color) -> Vec<Move> {
        let self_pieces = self
            .positions
            .iter()
            .filter_map(|(k, p)| if let Some(p) = p { Some((k, p)) } else { None })
            .filter(|(_, piece)| !piece.get_color().is_opponent(&self.turn, &self.team_mode));
        let moves = self_pieces.flat_map(|(k, p)| p.get_legal_moves(self, k));
        moves.collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    pub fn get_color(&self) -> Color {
        match self {
            Piece::Pawn(color) => *color,
            Piece::Knight(color) => *color,
            Piece::Bishop(color) => *color,
            Piece::Rook(color) => *color,
            Piece::Queen(color) => *color,
            Piece::King(color) => *color,
        }
    }

    pub fn get_hit_moves(&self, board: &Board, position: &Position) -> Vec<Move> {
        if let Some(piece) = board.positions.get(position) {
            if let Some(piece) = piece {
                if piece == self {
                    let mut moves = vec![];
                    match self {
                        Piece::Pawn(color) => {
                            let pm = color.get_pawn_direction();
                            let dirs = [-1, 1];
                            if pm.0 == 0 {
                                for d in dirs.iter() {
                                    let mut np = position.clone();
                                    np.0 += d;
                                    np.1 += pm.1;
                                    if board.is_position_capturable(&np, &self.get_color()) {
                                        moves.push(Move {
                                            from: *position,
                                            to: np,
                                            promotion: None,
                                        })
                                    }
                                }
                            } else if pm.1 == 0 {
                                for d in dirs.iter() {
                                    let mut np = position.clone();
                                    np.1 += d;
                                    np.0 += pm.0;
                                    if board.is_position_capturable(&np, &self.get_color()) {
                                        moves.push(Move {
                                            from: *position,
                                            to: np,
                                            promotion: None,
                                        })
                                    }
                                }
                            }
                        }
                        Piece::Knight(_)
                        | Piece::Bishop(_)
                        | Piece::Rook(_)
                        | Piece::Queen(_)
                        | Piece::King(_) => {
                            let move_moves = self.get_moveable_moves(board, position);
                            let capturable_moves = move_moves.into_iter().filter(|mov| {
                                board.is_position_capturable(&mov.to, &self.get_color())
                            });
                            moves.append(&mut capturable_moves.collect());
                        }
                    }
                    moves
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    pub fn get_legal_moves(&self, board: &Board, position: &Position) -> Vec<Move> {
        let mut moves = self.get_moveable_moves(board, position);
        moves.append(&mut self.get_hit_moves(board, position));
        moves.sort();
        moves.dedup();
        moves
            .into_iter()
            .filter(|mov| board.is_move_legal(mov))
            .collect()
    }

    pub fn get_moveable_moves(&self, board: &Board, position: &Position) -> Vec<Move> {
        if let Some(piece) = board.positions.get(position) {
            if let Some(piece) = piece {
                if piece == self {
                    let mut moves = vec![];
                    match self {
                        Piece::Pawn(_) => {
                            let pm = self.get_color().get_pawn_direction();
                            let ps = self.get_color().pawn_start(board);
                            let pm2 = {
                                if (ps.0 == 0 && ps.1 == position.1)
                                    || (ps.1 == 0 && ps.0 == position.0)
                                {
                                    let mut pm2 = pm.clone();
                                    pm2.0 *= 2;
                                    pm2.1 *= 2;
                                    pm2
                                } else {
                                    pm.clone()
                                }
                            };
                            let pms = vec![pm, pm2];
                            for pm in pms {
                                let mut np = position.clone();
                                np.0 += pm.0;
                                np.1 += pm.1;
                                if !board.is_position_occupied(&np) {
                                    moves.push(Move {
                                        from: *position,
                                        to: np,
                                        promotion: None,
                                    })
                                } else {
                                    break;
                                }
                            }
                        }
                        Piece::Knight(_) => {
                            let mvs = vec![
                                (1, 2),
                                (-1, 2),
                                (1, -2),
                                (-1, -2),
                                (2, 1),
                                (-2, 1),
                                (2, -1),
                                (-2, -1),
                            ];
                            for m in mvs.iter() {
                                let mut np = position.clone();
                                np.0 += m.0;
                                np.1 += m.1;
                                if board.is_position_moveable(&np, &self.get_color()) {
                                    moves.push(Move {
                                        from: *position,
                                        to: np,
                                        promotion: None,
                                    });
                                }
                            }
                        }
                        Piece::Bishop(_) => {
                            let dirs = [(1, 1), (-1, -1), (1, -1), (-1, 1)];
                            for dir in dirs.iter() {
                                let mut curpos = position.clone();
                                curpos.0 += dir.0;
                                curpos.1 += dir.1;
                                while board.is_position_moveable(&curpos, &self.get_color()) {
                                    moves.push(Move {
                                        from: position.clone(),
                                        to: curpos,
                                        promotion: None,
                                    });
                                    if board.is_position_occupied(&curpos) {
                                        break;
                                    }
                                    curpos.0 += dir.0;
                                    curpos.1 += dir.1;
                                }
                            }
                        }
                        Piece::Rook(_) => {
                            let dirs = [(1, 0), (-1, 0), (0, -1), (0, 1)];
                            for dir in dirs.iter() {
                                let mut curpos = position.clone();
                                curpos.0 += dir.0;
                                curpos.1 += dir.1;
                                while board.is_position_moveable(&curpos, &self.get_color()) {
                                    moves.push(Move {
                                        from: position.clone(),
                                        to: curpos,
                                        promotion: None,
                                    });
                                    if board.is_position_occupied(&curpos) {
                                        break;
                                    }
                                    curpos.0 += dir.0;
                                    curpos.1 += dir.1;
                                }
                            }
                        }
                        Piece::Queen(_) => {
                            let dirs = [
                                (1, 0),
                                (-1, 0),
                                (0, -1),
                                (0, 1),
                                (1, 1),
                                (-1, -1),
                                (1, -1),
                                (-1, 1),
                            ];
                            for dir in dirs.iter() {
                                let mut curpos = position.clone();
                                curpos.0 += dir.0;
                                curpos.1 += dir.1;
                                while board.is_position_moveable(&curpos, &self.get_color()) {
                                    moves.push(Move {
                                        from: position.clone(),
                                        to: curpos,
                                        promotion: None,
                                    });
                                    if board.is_position_occupied(&curpos) {
                                        break;
                                    }
                                    curpos.0 += dir.0;
                                    curpos.1 += dir.1;
                                }
                            }
                        }
                        Piece::King(_) => {
                            let dirs = [
                                (1, 0),
                                (-1, 0),
                                (0, -1),
                                (0, 1),
                                (1, 1),
                                (-1, -1),
                                (1, -1),
                                (-1, 1),
                            ];

                            for dir in dirs.iter() {
                                let mut curpos = position.clone();
                                curpos.0 += dir.0;
                                curpos.1 += dir.1;
                                if board.is_position_moveable(&curpos, &self.get_color()) {
                                    moves.push(Move {
                                        from: position.clone(),
                                        to: curpos,
                                        promotion: None,
                                    })
                                }
                            }
                        }
                    }
                    moves
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
    Red,
    Blue,
}

impl Color {
    pub fn is_opponent(&self, other: &Color, mode: &TeamMode) -> bool {
        match mode {
            TeamMode::Team => {
                (self.get_pawn_direction().0 != other.get_pawn_direction().0)
                    && (self.get_pawn_direction().1 != other.get_pawn_direction().1)
            }
            TeamMode::Solo => self != other,
        }
    }
    pub fn get_pawn_direction(&self) -> Position {
        match self {
            Color::White => (1, 0),
            Color::Black => (-1, 0),
            Color::Red => (0, -1),
            Color::Blue => (0, 1),
        }
    }
    pub fn pawn_start(&self, board: &Board) -> Position {
        match self {
            Color::White => (1, 0),
            Color::Black => ((board.width - 2) as i32, 0),
            Color::Red => (0, (board.width - 2) as i32),
            Color::Blue => (0, 1),
        }
    }
}
