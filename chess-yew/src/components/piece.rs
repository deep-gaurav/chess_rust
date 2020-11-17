use yew::prelude::*;

use chess_rust::*;

pub trait ToSVG {
    fn to_svg(&self) -> Html;
}

impl ToSVG for Piece {
    fn to_svg(&self) -> Html {
        match self {
            Piece::Pawn(color) => match color {
                Color::White => {
                    html! {
                        <svg class="piece piece-white" viewBox="0 0 24.5 32" version="1.1" >
                            <g transform="translate(-9.75,-8.25)">
                            <path d="m22 9c-2.21 0-4 1.79-4 4 0 0.89 0.29 1.71 0.78 2.38-1.95 1.12-3.28 3.21-3.28 5.62 0 2.03 0.94 3.84 2.41 5.03-3 1.06-7.41 5.55-7.41 13.47h23c0-7.92-4.41-12.41-7.41-13.47 1.47-1.19 2.41-3 2.41-5.03 0-2.41-1.33-4.5-3.28-5.62 0.49-0.67 0.78-1.49 0.78-2.38 0-2.21-1.79-4-4-4z"  stroke="#000" stroke-linecap="round" stroke-width="1.5"/>
                            </g>
                        </svg>
                    }
                }
                Color::Black => {
                    html! {
                        <svg class="piece piece-black" viewBox="0 0 24.5 32" version="1.1" >
                            <g transform="translate(-9.75,-8.25)">
                            <path d="m22 9c-2.21 0-4 1.79-4 4 0 0.89 0.29 1.71 0.78 2.38-1.95 1.12-3.28 3.21-3.28 5.62 0 2.03 0.94 3.84 2.41 5.03-3 1.06-7.41 5.55-7.41 13.47h23c0-7.92-4.41-12.41-7.41-13.47 1.47-1.19 2.41-3 2.41-5.03 0-2.41-1.33-4.5-3.28-5.62 0.49-0.67 0.78-1.49 0.78-2.38 0-2.21-1.79-4-4-4z" stroke="#000" stroke-linecap="round" stroke-width="1.5"/>
                            </g>
                        </svg>
                    }
                }
                Color::Red => {
                    html! {
                        <svg class="piece piece-red" viewBox="0 0 24.5 32" version="1.1" >
                            <g transform="translate(-9.75,-8.25)">
                            <path d="m22 9c-2.21 0-4 1.79-4 4 0 0.89 0.29 1.71 0.78 2.38-1.95 1.12-3.28 3.21-3.28 5.62 0 2.03 0.94 3.84 2.41 5.03-3 1.06-7.41 5.55-7.41 13.47h23c0-7.92-4.41-12.41-7.41-13.47 1.47-1.19 2.41-3 2.41-5.03 0-2.41-1.33-4.5-3.28-5.62 0.49-0.67 0.78-1.49 0.78-2.38 0-2.21-1.79-4-4-4z"  stroke="#000" stroke-linecap="round" stroke-width="1.5"/>
                            </g>
                        </svg>
                    }
                }
                Color::Blue => {
                    html! {
                        <svg class="piece piece-blue" viewBox="0 0 24.5 32" version="1.1" >
                            <g transform="translate(-9.75,-8.25)">
                            <path d="m22 9c-2.21 0-4 1.79-4 4 0 0.89 0.29 1.71 0.78 2.38-1.95 1.12-3.28 3.21-3.28 5.62 0 2.03 0.94 3.84 2.41 5.03-3 1.06-7.41 5.55-7.41 13.47h23c0-7.92-4.41-12.41-7.41-13.47 1.47-1.19 2.41-3 2.41-5.03 0-2.41-1.33-4.5-3.28-5.62 0.49-0.67 0.78-1.49 0.78-2.38 0-2.21-1.79-4-4-4z"  stroke="#000" stroke-linecap="round" stroke-width="1.5"/>
                            </g>
                        </svg>
                    }
                }
            },
            Piece::Knight(color) => match color {
                Color::White => {
                    html! {<svg class="piece piece-white" viewBox="0 0 33.529 33.5" version="1.1" >
                    <g transform="translate(-5.25 -6.25)" fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <path d="m22 10c10.5 1 16.5 8 16 29h-23c0-9 10-6.5 8-21" />
                      <path d="m24 18c0.38 2.91-5.55 7.37-8 9-3 2-2.82 4.34-5 4-1.042-0.94 1.41-3.04 0-3-1 0 0.19 1.23-1 2-1 0-4.003 1-4-4 0-2 6-12 6-12s1.89-1.9 2-3.5c-0.73-0.994-0.5-2-0.5-3 1-1 3 2.5 3 2.5h2s0.78-1.992 2.5-3c1 0 1 3 1 3" />
                      <path d="m9.5 25.5a0.5 0.5 0 1 1-1 0 0.5 0.5 0 1 1 1 0z"/>
                      <path transform="matrix(.866 .5 -.5 .866 9.693 -5.173)" d="m15 15.5a0.5 1.5 0 1 1-1 0 0.5 1.5 0 1 1 1 0z"/>
                     </g>
                    </svg>
                    }
                }
                Color::Black => {
                    html! {

                    <svg class="piece piece-black" viewBox="0 0 33.529 33.5" version="1.1">
                    <g transform="translate(-5.25 -6.25)" fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                     <path d="m22 10c10.5 1 16.5 8 16 29h-23c0-9 10-6.5 8-21" stroke="#000"/>
                     <path d="m24 18c0.38 2.91-5.55 7.37-8 9-3 2-2.82 4.34-5 4-1.042-0.94 1.41-3.04 0-3-1 0 0.19 1.23-1 2-1 0-4.003 1-4-4 0-2 6-12 6-12s1.89-1.9 2-3.5c-0.73-0.994-0.5-2-0.5-3 1-1 3 2.5 3 2.5h2s0.78-1.992 2.5-3c1 0 1 3 1 3" stroke="#000"/>
                     <g >
                      <path d="m9.5 25.5a0.5 0.5 0 1 1-1 0 0.5 0.5 0 1 1 1 0z" stroke="#fff"/>
                      <path transform="matrix(.866 .5 -.5 .866 9.693 -5.173)" d="m15 15.5a0.5 1.5 0 1 1-1 0 0.5 1.5 0 1 1 1 0z" stroke="#fff"/>
                      <path d="m24.55 10.4-0.45 1.45 0.5 0.15c3.15 1 5.65 2.49 7.9 6.75s3.25 10.31 2.75 20.25l-0.05 0.5h2.25l0.05-0.5c0.5-10.06-0.88-16.85-3.25-21.34s-5.79-6.64-9.19-7.16z" stroke="none"/>
                     </g>
                    </g>
                    </svg>

                    }
                }
                Color::Red => {
                    html! {<svg class="piece piece-red" viewBox="0 0 33.529 33.5" version="1.1" >
                    <g transform="translate(-5.25 -6.25)" fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <path d="m22 10c10.5 1 16.5 8 16 29h-23c0-9 10-6.5 8-21" />
                      <path d="m24 18c0.38 2.91-5.55 7.37-8 9-3 2-2.82 4.34-5 4-1.042-0.94 1.41-3.04 0-3-1 0 0.19 1.23-1 2-1 0-4.003 1-4-4 0-2 6-12 6-12s1.89-1.9 2-3.5c-0.73-0.994-0.5-2-0.5-3 1-1 3 2.5 3 2.5h2s0.78-1.992 2.5-3c1 0 1 3 1 3" />
                      <path d="m9.5 25.5a0.5 0.5 0 1 1-1 0 0.5 0.5 0 1 1 1 0z"/>
                      <path transform="matrix(.866 .5 -.5 .866 9.693 -5.173)" d="m15 15.5a0.5 1.5 0 1 1-1 0 0.5 1.5 0 1 1 1 0z"/>
                     </g>
                    </svg>
                    }
                }
                Color::Blue => {
                    html! {<svg class="piece piece-blue" viewBox="0 0 33.529 33.5" version="1.1" >
                    <g transform="translate(-5.25 -6.25)" fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <path d="m22 10c10.5 1 16.5 8 16 29h-23c0-9 10-6.5 8-21" />
                      <path d="m24 18c0.38 2.91-5.55 7.37-8 9-3 2-2.82 4.34-5 4-1.042-0.94 1.41-3.04 0-3-1 0 0.19 1.23-1 2-1 0-4.003 1-4-4 0-2 6-12 6-12s1.89-1.9 2-3.5c-0.73-0.994-0.5-2-0.5-3 1-1 3 2.5 3 2.5h2s0.78-1.992 2.5-3c1 0 1 3 1 3" />
                      <path d="m9.5 25.5a0.5 0.5 0 1 1-1 0 0.5 0.5 0 1 1 1 0z"/>
                      <path transform="matrix(.866 .5 -.5 .866 9.693 -5.173)" d="m15 15.5a0.5 1.5 0 1 1-1 0 0.5 1.5 0 1 1 1 0z"/>
                     </g>
                    </svg>
                    }
                }
            },
            Piece::Bishop(color) => match color {
                Color::White => {
                    html! {
                                        <svg class="piece piece-white" viewBox="0 0 34.5 34.816" version="1.1" >
                     <g transform="translate(-5.25 -4.75)" fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <g  stroke-linecap="butt">
                       <path d="m9 36c3.39-0.97 10.11 0.43 13.5-2 3.39 2.43 10.11 1.03 13.5 2 0 0 1.65 0.54 3 2-0.68 0.97-1.65 0.99-3 0.5-3.39-0.97-10.11 0.46-13.5-1-3.39 1.46-10.11 0.03-13.5 1-1.354 0.49-2.323 0.47-3-0.5 1.354-1.94 3-2 3-2z"/>
                       <path d="m15 32c2.5 2.5 12.5 2.5 15 0 0.5-1.5 0-2 0-2 0-2.5-2.5-4-2.5-4 5.5-1.5 6-11.5-5-15.5-11 4-10.5 14-5 15.5 0 0-2.5 1.5-2.5 4 0 0-0.5 0.5 0 2z"/>
                       <path d="m25 8a2.5 2.5 0 1 1-5 0 2.5 2.5 0 1 1 5 0z"/>
                      </g>
                      <path d="m17.5 26h10m-12.5 4h15m-7.5-14.5v5m-2.5-2.5h5" fill="none" stroke-linejoin="miter"/>
                     </g>
                    </svg>

                                    }
                }
                Color::Black => {
                    html! {
                                        <svg class="piece piece-black" viewBox="0 0 34.5 34.816" version="1.1">

                     <g transform="translate(-5.25 -4.75)" fill-rule="evenodd" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <g stroke="#000" stroke-linecap="butt">
                       <path d="m9 36c3.39-0.97 10.11 0.43 13.5-2 3.39 2.43 10.11 1.03 13.5 2 0 0 1.65 0.54 3 2-0.68 0.97-1.65 0.99-3 0.5-3.39-0.97-10.11 0.46-13.5-1-3.39 1.46-10.11 0.03-13.5 1-1.354 0.49-2.323 0.47-3-0.5 1.354-1.94 3-2 3-2z"/>
                       <path d="m15 32c2.5 2.5 12.5 2.5 15 0 0.5-1.5 0-2 0-2 0-2.5-2.5-4-2.5-4 5.5-1.5 6-11.5-5-15.5-11 4-10.5 14-5 15.5 0 0-2.5 1.5-2.5 4 0 0-0.5 0.5 0 2z"/>
                       <path d="m25 8a2.5 2.5 0 1 1-5 0 2.5 2.5 0 1 1 5 0z"/>
                      </g>
                      <path d="m17.5 26h10m-12.5 4h15m-7.5-14.5v5m-2.5-2.5h5" fill="none" stroke="#fff" stroke-linejoin="miter"/>
                     </g>
                    </svg>

                                    }
                }
                Color::Red => {
                    html! {
                                        <svg class="piece piece-red" viewBox="0 0 34.5 34.816" version="1.1" >
                     <g transform="translate(-5.25 -4.75)" fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <g  stroke-linecap="butt">
                       <path d="m9 36c3.39-0.97 10.11 0.43 13.5-2 3.39 2.43 10.11 1.03 13.5 2 0 0 1.65 0.54 3 2-0.68 0.97-1.65 0.99-3 0.5-3.39-0.97-10.11 0.46-13.5-1-3.39 1.46-10.11 0.03-13.5 1-1.354 0.49-2.323 0.47-3-0.5 1.354-1.94 3-2 3-2z"/>
                       <path d="m15 32c2.5 2.5 12.5 2.5 15 0 0.5-1.5 0-2 0-2 0-2.5-2.5-4-2.5-4 5.5-1.5 6-11.5-5-15.5-11 4-10.5 14-5 15.5 0 0-2.5 1.5-2.5 4 0 0-0.5 0.5 0 2z"/>
                       <path d="m25 8a2.5 2.5 0 1 1-5 0 2.5 2.5 0 1 1 5 0z"/>
                      </g>
                      <path d="m17.5 26h10m-12.5 4h15m-7.5-14.5v5m-2.5-2.5h5" fill="none" stroke-linejoin="miter"/>
                     </g>
                    </svg>

                                    }
                }
                Color::Blue => {
                    html! {
                                        <svg class="piece piece-blue" viewBox="0 0 34.5 34.816" version="1.1" >
                     <g transform="translate(-5.25 -4.75)" fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <g  stroke-linecap="butt">
                       <path d="m9 36c3.39-0.97 10.11 0.43 13.5-2 3.39 2.43 10.11 1.03 13.5 2 0 0 1.65 0.54 3 2-0.68 0.97-1.65 0.99-3 0.5-3.39-0.97-10.11 0.46-13.5-1-3.39 1.46-10.11 0.03-13.5 1-1.354 0.49-2.323 0.47-3-0.5 1.354-1.94 3-2 3-2z"/>
                       <path d="m15 32c2.5 2.5 12.5 2.5 15 0 0.5-1.5 0-2 0-2 0-2.5-2.5-4-2.5-4 5.5-1.5 6-11.5-5-15.5-11 4-10.5 14-5 15.5 0 0-2.5 1.5-2.5 4 0 0-0.5 0.5 0 2z"/>
                       <path d="m25 8a2.5 2.5 0 1 1-5 0 2.5 2.5 0 1 1 5 0z"/>
                      </g>
                      <path d="m17.5 26h10m-12.5 4h15m-7.5-14.5v5m-2.5-2.5h5" fill="none" stroke-linejoin="miter"/>
                     </g>
                    </svg>

                                    }
                }
            },
            Piece::Rook(color) => match color {
                Color::White => {
                    html! {
                                       <svg class="piece piece-white" viewBox="0 0 28.5 31.5" version="1.1">
                    <g transform="translate(-8.25,-8.25)"  fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                     <g stroke-linecap="butt">
                      <path d="m9 39h27v-3h-27z"/>
                      <path d="m12 36v-4h21v4z"/>
                      <path d="m11 14v-5h4v2h5v-2h5v2h5v-2h4v5"/>
                     </g>
                     <path d="m34 14-3 3h-17l-3-3"/>
                     <path d="m31 17v12.5h-17v-12.5" stroke-linecap="butt" stroke-linejoin="miter"/>
                     <path d="m31 29.5 1.5 2.5h-20l1.5-2.5"/>
                     <path d="m11 14h23" fill="none" stroke="#000" stroke-linejoin="miter"/>
                    </g>
                    </svg>
                                   }
                }
                Color::Black => {
                    html! {
                                            <svg class="piece piece-black" viewBox="0 0 28.5 31.5" version="1.1">
                     <g transform="translate(-8.25,-8.25)" fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <g stroke-linecap="butt">
                       <path d="m9 39h27v-3h-27z"/>
                       <path d="m12.5 32 1.5-2.5h17l1.5 2.5z"/>
                       <path d="m12 36v-4h21v4z"/>
                       <path d="m14 29.5v-13h17v13z" stroke-linejoin="miter"/>
                       <path d="m14 16.5-3-2.5h23l-3 2.5z"/>
                       <path d="m11 14v-5h4v2h5v-2h5v2h5v-2h4v5z"/>
                      </g>
                      <g fill="none" stroke="#fff" stroke-linejoin="miter" stroke-width="1">
                       <path d="m12 35.5h21"/>
                       <path d="m13 31.5h19"/>
                       <path d="m14 29.5h17"/>
                       <path d="m14 16.5h17"/>
                       <path d="m11 14h23"/>
                      </g>
                     </g>
                    </svg>

                                        }
                }
                Color::Red => {
                    html! {
                                       <svg class="piece piece-red" viewBox="0 0 28.5 31.5" version="1.1">
                    <g transform="translate(-8.25,-8.25)"  fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                     <g stroke-linecap="butt">
                      <path d="m9 39h27v-3h-27z"/>
                      <path d="m12 36v-4h21v4z"/>
                      <path d="m11 14v-5h4v2h5v-2h5v2h5v-2h4v5"/>
                     </g>
                     <path d="m34 14-3 3h-17l-3-3"/>
                     <path d="m31 17v12.5h-17v-12.5" stroke-linecap="butt" stroke-linejoin="miter"/>
                     <path d="m31 29.5 1.5 2.5h-20l1.5-2.5"/>
                     <path d="m11 14h23" fill="none" stroke="#000" stroke-linejoin="miter"/>
                    </g>
                    </svg>
                                   }
                }
                Color::Blue => {
                    html! {
                                       <svg class="piece piece-blue" viewBox="0 0 28.5 31.5" version="1.1">
                    <g transform="translate(-8.25,-8.25)"  fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                     <g stroke-linecap="butt">
                      <path d="m9 39h27v-3h-27z"/>
                      <path d="m12 36v-4h21v4z"/>
                      <path d="m11 14v-5h4v2h5v-2h5v2h5v-2h4v5"/>
                     </g>
                     <path d="m34 14-3 3h-17l-3-3"/>
                     <path d="m31 17v12.5h-17v-12.5" stroke-linecap="butt" stroke-linejoin="miter"/>
                     <path d="m31 29.5 1.5 2.5h-20l1.5-2.5"/>
                     <path d="m11 14h23" fill="none" stroke="#000" stroke-linejoin="miter"/>
                    </g>
                    </svg>
                                   }
                }
            },
            Piece::Queen(color) => match color {
                Color::White => {
                    html! {<svg class="piece piece-white" viewBox="0 0 38.5 35.254" version="1.1" >

                     <g transform="translate(-3.25 -4.75)"  fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <path transform="translate(-1,-1)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path transform="translate(15.5,-5.5)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path transform="translate(32,-1)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path transform="translate(7,-4.5)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path transform="translate(24,-4)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path d="m9 26c8.5-1.5 21-1.5 27 0l2-12-7 11v-14l-5.5 13.5-3-15-3 15-5.5-14v14.5l-7-11z" stroke-linecap="butt"/>
                      <path d="m9 26c0 2 1.5 2 2.5 4 1 1.5 1 1 0.5 3.5-1.5 1-1.5 2.5-1.5 2.5-1.5 1.5 0.5 2.5 0.5 2.5 6.5 1 16.5 1 23 0 0 0 1.5-1 0-2.5 0 0 0.5-1.5-1-2.5-0.5-2.5-0.5-2 0.5-3.5 1-2 2.5-2 2.5-4-8.5-1.5-18.5-1.5-27 0z" stroke-linecap="butt"/>
                      <path d="m11.5 30c3.5-1 18.5-1 22 0" fill="none"/>
                      <path d="m12 33.5c6-1 15-1 21 0" fill="none"/>
                     </g>
                    </svg>
                    }
                }
                Color::Black => {
                    html! {<svg class="piece piece-black" viewBox="0 0 38.5 35.943" version="1.1">
                     <g transform="translate(-3.25,-5.25)" fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <g stroke="none">
                       <circle cx="6" cy="12" r="2.75"/>
                       <circle cx="14" cy="9" r="2.75"/>
                       <circle cx="22.5" cy="8" r="2.75"/>
                       <circle cx="31" cy="9" r="2.75"/>
                       <circle cx="39" cy="12" r="2.75"/>
                      </g>
                      <path d="m9 26c8.5-1.5 21-1.5 27 0l2.5-12.5-7.5 11.5-0.3-14.1-5.2 13.6-3-14.5-3 14.5-5.2-13.6-0.3 14.1-7.5-11.5z" stroke="#000" stroke-linecap="butt"/>
                      <path d="m9 26c0 2 1.5 2 2.5 4 1 1.5 1 1 0.5 3.5-1.5 1-1.5 2.5-1.5 2.5-1.5 1.5 0.5 2.5 0.5 2.5 6.5 1 16.5 1 23 0 0 0 1.5-1 0-2.5 0 0 0.5-1.5-1-2.5-0.5-2.5-0.5-2 0.5-3.5 1-2 2.5-2 2.5-4-8.5-1.5-18.5-1.5-27 0z" stroke-linecap="butt"/>
                      <g fill="none">
                       <path d="m11 38.5a35 35 1 0 0 23 0" stroke="#000" stroke-linecap="butt"/>
                       <g stroke="#fff">
                        <path d="m11 29a35 35 1 0 1 23 0"/>
                        <path d="m12.5 31.5h20"/>
                        <path d="m11.5 34.5a35 35 1 0 0 22 0"/>
                        <path d="m10.5 37.5a35 35 1 0 0 24 0"/>
                       </g>
                      </g>
                     </g>
                    </svg>}
                }
                Color::Red => {
                    html! {<svg class="piece piece-red" viewBox="0 0 38.5 35.254" version="1.1" >

                     <g transform="translate(-3.25 -4.75)"  fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <path transform="translate(-1,-1)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path transform="translate(15.5,-5.5)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path transform="translate(32,-1)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path transform="translate(7,-4.5)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path transform="translate(24,-4)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path d="m9 26c8.5-1.5 21-1.5 27 0l2-12-7 11v-14l-5.5 13.5-3-15-3 15-5.5-14v14.5l-7-11z" stroke-linecap="butt"/>
                      <path d="m9 26c0 2 1.5 2 2.5 4 1 1.5 1 1 0.5 3.5-1.5 1-1.5 2.5-1.5 2.5-1.5 1.5 0.5 2.5 0.5 2.5 6.5 1 16.5 1 23 0 0 0 1.5-1 0-2.5 0 0 0.5-1.5-1-2.5-0.5-2.5-0.5-2 0.5-3.5 1-2 2.5-2 2.5-4-8.5-1.5-18.5-1.5-27 0z" stroke-linecap="butt"/>
                      <path d="m11.5 30c3.5-1 18.5-1 22 0" fill="none"/>
                      <path d="m12 33.5c6-1 15-1 21 0" fill="none"/>
                     </g>
                    </svg>
                    }
                }
                Color::Blue => {
                    html! {<svg class="piece piece-blue" viewBox="0 0 38.5 35.254" version="1.1" >

                     <g transform="translate(-3.25 -4.75)"  fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <path transform="translate(-1,-1)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path transform="translate(15.5,-5.5)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path transform="translate(32,-1)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path transform="translate(7,-4.5)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path transform="translate(24,-4)" d="m9 13a2 2 0 1 1-4 0 2 2 0 1 1 4 0z"/>
                      <path d="m9 26c8.5-1.5 21-1.5 27 0l2-12-7 11v-14l-5.5 13.5-3-15-3 15-5.5-14v14.5l-7-11z" stroke-linecap="butt"/>
                      <path d="m9 26c0 2 1.5 2 2.5 4 1 1.5 1 1 0.5 3.5-1.5 1-1.5 2.5-1.5 2.5-1.5 1.5 0.5 2.5 0.5 2.5 6.5 1 16.5 1 23 0 0 0 1.5-1 0-2.5 0 0 0.5-1.5-1-2.5-0.5-2.5-0.5-2 0.5-3.5 1-2 2.5-2 2.5-4-8.5-1.5-18.5-1.5-27 0z" stroke-linecap="butt"/>
                      <path d="m11.5 30c3.5-1 18.5-1 22 0" fill="none"/>
                      <path d="m12 33.5c6-1 15-1 21 0" fill="none"/>
                     </g>
                    </svg>
                    }
                }
            },
            Piece::King(color) => match color {
                Color::White => {
                    html! {
                                        <svg class="piece piece-white" viewBox="0 0 34.775 35.125" version="1.1">
                     <g transform="translate(-5.0809 -5.25)" fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <g stroke-linejoin="miter">
                       <path d="M 22.5,11.63 V 6" fill="none"/>
                       <path d="m20 8h5" fill="none"/>
                       <path d="m22.5 25s4.5-7.5 3-10.5c0 0-1-2.5-3-2.5s-3 2.5-3 2.5c-1.5 3 3 10.5 3 10.5"  stroke-linecap="butt"/>
                      </g>
                      <path d="m11.5 37c5.5 3.5 15.5 3.5 21 0v-7s9-4.5 6-10.5c-4-6.5-13.5-3.5-16 4v3.5-3.5c-3.5-7.5-13-10.5-16-4-3 6 5 10 5 10z" />
                      <g fill="none">
                       <path d="m11.5 30c5.5-3 15.5-3 21 0"/>
                       <path d="m11.5 33.5c5.5-3 15.5-3 21 0"/>
                       <path d="m11.5 37c5.5-3 15.5-3 21 0"/>
                      </g>
                     </g>
                    </svg>
                                    }
                }
                Color::Black => {
                    html! {

                       <svg class="piece piece-black" viewBox="0 0 38.5 35.943" version="1.1">

                       <g
                       style="fill:none;fill-opacity:1;fill-rule:evenodd;stroke:#000000;stroke-width:1.5;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"
                       transform="translate(-5.0809061,-5.25)"
                       id="g92">
                      <path
                         d="M 22.5,11.63 V 6"
                         style="fill:none;stroke:#000000;stroke-linejoin:miter"
                         id="path80" />
                      <path
                         d="m 22.5,25 c 0,0 4.5,-7.5 3,-10.5 0,0 -1,-2.5 -3,-2.5 -2,0 -3,2.5 -3,2.5 -1.5,3 3,10.5 3,10.5"
                         style="fill:#000000;fill-opacity:1;stroke-linecap:butt;stroke-linejoin:miter"
                         id="path82" />
                      <path
                         d="m 11.5,37 c 5.5,3.5 15.5,3.5 21,0 v -7 c 0,0 9,-4.5 6,-10.5 -4,-6.5 -13.5,-3.5 -16,4 V 27 23.5 C 19,16 9.5,13 6.5,19.5 c -3,6 5,10 5,10 z"
                         style="fill:#000000;stroke:#000000"
                         id="path84" />
                      <path
                         d="m 20,8 h 5"
                         style="fill:none;stroke:#000000;stroke-linejoin:miter"
                         id="path86" />
                      <path
                         d="m 32,29.5 c 0,0 8.5,-4 6.03,-9.65 C 34.15,14 25,18 22.5,24.5 l 0.01,2.1 -0.01,-2.1 C 20,18 9.906,14 6.997,19.85 c -2.497,5.65 4.853,9 4.853,9"
                         style="fill:none;stroke:#ffffff"
                         id="path88" />
                      <path
                         d="m 11.5,30 c 5.5,-3 15.5,-3 21,0 m -21,3.5 c 5.5,-3 15.5,-3 21,0 m -21,3.5 c 5.5,-3 15.5,-3 21,0"
                         style="fill:none;stroke:#ffffff"
                         id="path90" />
                    </g>
                       </svg>

                                       }
                }
                Color::Red => {
                    html! {
                                        <svg class="piece piece-red" viewBox="0 0 34.775 35.125" version="1.1">
                     <g transform="translate(-5.0809 -5.25)" fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <g stroke-linejoin="miter">
                       <path d="M 22.5,11.63 V 6" fill="none"/>
                       <path d="m20 8h5" fill="none"/>
                       <path d="m22.5 25s4.5-7.5 3-10.5c0 0-1-2.5-3-2.5s-3 2.5-3 2.5c-1.5 3 3 10.5 3 10.5"  stroke-linecap="butt"/>
                      </g>
                      <path d="m11.5 37c5.5 3.5 15.5 3.5 21 0v-7s9-4.5 6-10.5c-4-6.5-13.5-3.5-16 4v3.5-3.5c-3.5-7.5-13-10.5-16-4-3 6 5 10 5 10z" />
                      <g fill="none">
                       <path d="m11.5 30c5.5-3 15.5-3 21 0"/>
                       <path d="m11.5 33.5c5.5-3 15.5-3 21 0"/>
                       <path d="m11.5 37c5.5-3 15.5-3 21 0"/>
                      </g>
                     </g>
                    </svg>
                                    }
                }
                Color::Blue => {
                    html! {
                                        <svg class="piece piece-blue" viewBox="0 0 34.775 35.125" version="1.1">
                     <g transform="translate(-5.0809 -5.25)" fill-rule="evenodd" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                      <g stroke-linejoin="miter">
                       <path d="M 22.5,11.63 V 6" fill="none"/>
                       <path d="m20 8h5" fill="none"/>
                       <path d="m22.5 25s4.5-7.5 3-10.5c0 0-1-2.5-3-2.5s-3 2.5-3 2.5c-1.5 3 3 10.5 3 10.5"  stroke-linecap="butt"/>
                      </g>
                      <path d="m11.5 37c5.5 3.5 15.5 3.5 21 0v-7s9-4.5 6-10.5c-4-6.5-13.5-3.5-16 4v3.5-3.5c-3.5-7.5-13-10.5-16-4-3 6 5 10 5 10z" />
                      <g fill="none">
                       <path d="m11.5 30c5.5-3 15.5-3 21 0"/>
                       <path d="m11.5 33.5c5.5-3 15.5-3 21 0"/>
                       <path d="m11.5 37c5.5-3 15.5-3 21 0"/>
                      </g>
                     </g>
                    </svg>
                                    }
                }
            },
        }
    }
}
