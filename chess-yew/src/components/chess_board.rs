use super::piece::ToSVG;
use chess_rust::*;
use yew::prelude::*;

pub struct ChessBoard {
    pub board: Board,
    pub link: ComponentLink<Self>,
    pub highlighted_positions: Vec<Position>,
    pub available_moves: Vec<Move>,
    pub self_color: Color,
    pub move_callback: Callback<Move>,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub board: Board,
    pub self_color: Color,
    pub movecb: Callback<Move>,
}

pub enum Msg {
    PositionClick(Position),
}

impl Component for ChessBoard {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            board: props.board,
            self_color: props.self_color,
            link,
            highlighted_positions: vec![],
            move_callback: props.movecb,
            available_moves: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PositionClick(position) => {
                if self.board.turn != self.self_color {
                    self.highlighted_positions.clear();
                    return true;
                }
                self.highlighted_positions.clear();
                self.highlighted_positions.push(position);

                if let Some(mov) = self.available_moves.iter().find(|m| m.to == position) {
                    self.move_callback.emit(mov.clone());
                } else if let Some(piece) = self.board.positions.get(&position) {
                    if let Some(piece) = piece {
                        self.highlighted_positions.extend(
                            piece
                                .get_legal_moves(&self.board, &position)
                                .into_iter()
                                .map(|mov| mov.to),
                        );
                        self.available_moves.clear();
                        self.available_moves
                            .append(&mut piece.get_legal_moves(&self.board, &position));
                    }
                }

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        if self.board != _props.board {
            self.board = _props.board;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut positions = Vec::new();
        for i in 0..self.board.width {
            for j in 0..self.board.width {
                let pos = self.board.positions.get(&(i as i32, j as i32));
                if let Some(pos) = pos {
                    positions.push(self.view_position(&(i as i32, j as i32), pos));
                } else {
                    positions.push(html! {<div />})
                }
            }
        }
        html! {
            <div class="boardback">
            <div class="board"
                style={format!("grid-template-columns: {}; transform: rotate({}deg);","auto ".repeat(self.board.width as usize),self.self_color.get_rotation())}
            >
                {
                    for positions
                }
            </div>
            </div>
        }
    }
}

impl ChessBoard {
    pub fn view_position(&self, position: &Position, piece: &Option<Piece>) -> Html {
        let position = position.clone();
        let piece = piece.clone();
        let is_highlight = self.highlighted_positions.contains(&position);
        let is_check = {
            if let Some(piece) = piece {
                if let Piece::King(color) = piece {
                    if self.board.is_check(color) {
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
        };
        html! {
            <div class={
                if Board::get_position_color(&position) == Color::White{
                    "white"
                }else{
                    "black"
                }
            }>
                <div class="position" onclick=self.link.callback(move|_|Msg::PositionClick(position.clone()))>
                    {
                        if is_check{
                            html!{
                                <div class="check_highlight">
                                </div>
                            }
                        }else{
                            html!{}
                        }
                    }

                    {
                        if is_highlight{
                            html!{
                                <div class="highlight">
                                </div>
                            }
                        }else{
                            html!{}
                        }
                    }
                    {
                        if let Some(piece)=piece{
                            html!{
                                <div class="piececontainer" style={format!("transform:rotate(-{}deg)",self.self_color.get_rotation())} >
                                    {piece.to_svg()}
                                </div>
                            }
                        }else{
                            html!{}
                        }
                    }
                </div>
            </div>
        }
    }
}

trait GetRotation {
    fn get_rotation(&self) -> i32;
}

impl GetRotation for Color {
    fn get_rotation(&self) -> i32 {
        match self {
            Color::White => 180,
            Color::Black => 0,
            Color::Red => 90,
            Color::Blue => 270,
        }
    }
}
