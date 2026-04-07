//! 游戏规则模块

pub mod mahjong;
pub mod card_games;
pub mod board_games;
pub mod doudizhu;
pub mod blackjack;
pub mod bridge;
pub mod sudoku;
pub mod rubiks_cube;
pub mod pao_de_kuai;
pub mod sheng_ji;
pub mod aeroplane_chess;
pub mod chinese_checkers;

pub use mahjong::*;
pub use doudizhu::DouDiZhuRules;
pub use blackjack::BlackjackRules;
pub use bridge::BridgeRules;
pub use sudoku::SudokuRules;
pub use rubiks_cube::{RubiksCubeRules, CubeType};
pub use pao_de_kuai::PaoDeKuaiRules;
pub use sheng_ji::ShengJiRules;
pub use aeroplane_chess::AeroplaneChessRules;
pub use chinese_checkers::ChineseCheckersRules;