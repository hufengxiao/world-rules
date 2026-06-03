//! 棋类游戏规则

pub mod chess;
pub mod chinese_chess;
pub mod go;
pub mod gomoku;

pub use chess::ChessRules;
pub use chinese_chess::ChineseChessRules;
pub use go::GoRules;
pub use gomoku::GomokuRules;
