//! 棋类游戏规则

pub mod chess;
pub mod chess_variants;
pub mod chinese_chess;
pub mod go;
pub mod gomoku;

pub use chess::ChessRules;
pub use chess_variants::{
    ChessVariant, ChessVariantsRules, InternationalChessRules, JanggiVariantRules,
    ShogiVariantRules,
};
pub use chinese_chess::ChineseChessRules;
pub use go::{
    GoRules, GoVariant, IngRules, KoRule, NewZealandRules, ScoringMethod, Stone, TimeSystem,
    WMSGRules,
};
pub use gomoku::GomokuRules;
