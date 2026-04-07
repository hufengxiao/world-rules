//! 棋类游戏规则

pub mod go;
pub mod chinese_chess;
pub mod chess;
pub mod gomoku;

pub use go::GoRules;
pub use chinese_chess::ChineseChessRules;
pub use chess::ChessRules;
pub use gomoku::GomokuRules;