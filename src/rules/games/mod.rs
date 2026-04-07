//! 游戏规则模块

pub mod mahjong;
pub mod card_games;
pub mod board_games;
pub mod doudizhu;
pub mod blackjack;

pub use mahjong::*;
pub use doudizhu::DouDiZhuRules;
pub use blackjack::BlackjackRules;