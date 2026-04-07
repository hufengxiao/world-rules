//! 游戏规则模块

pub mod mahjong;
pub mod card_games;
pub mod board_games;
pub mod doudizhu;
pub mod blackjack;
pub mod bridge;
pub mod sudoku;
pub mod rubiks_cube;

pub use mahjong::*;
pub use doudizhu::DouDiZhuRules;
pub use blackjack::BlackjackRules;
pub use bridge::BridgeRules;
pub use sudoku::SudokuRules;
pub use rubiks_cube::{RubiksCubeRules, CubeType};