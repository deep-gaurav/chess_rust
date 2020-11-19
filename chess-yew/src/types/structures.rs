use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use chess_rust::*;

use log::{error, warn};

use std::collections::HashSet;

#[derive(Deserialize, Debug, Clone)]
pub struct Lobby {
    pub id: String,
    pub players: HashMap<String, Player>,
    pub state: State,
}

#[derive(Debug, Deserialize, Clone)]
pub enum State {
    Lobby(String),
    Game(Board),
}
impl State {
    pub fn leader(&self) -> String {
        match self {
            State::Lobby(id) => id.clone(),
            State::Game(_) => {
                //FIXME: do this
                "".into()
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub status: PlayerStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerStatus {
    Initiated,
    JoinedLobby(String, Color),
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum CloseCodes {
    WrongInit,
    CantCreateLobby,
    CantJoinLobbyDoestExist,
    NewSessionOpened,
    LobbyFull,
}
impl std::fmt::Display for CloseCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerMessage {
    Initialize(String, String),
    JoinLobby(String),
    CreateLobby,
    Ping,
    Move(Move),

    StartGame(GameType, TeamMode),
}

#[derive(Debug, Deserialize, Clone)]
pub enum SocketMessage {
    LobbyJoined(Lobby, Color),
    PlayerJoined(Player, Color),
    PlayerDisconnected(Player),
    Close(CloseCodes),

    Moved(Board, Move),

    LeaderChange(State),
    GameStart(State),

    Pong,
}
