//! 游戏规则模块

pub mod aeroplane_chess;
pub mod blackjack;
pub mod board_games;
pub mod bridge;
pub mod card_games;
pub mod chinese_checkers;
pub mod domino;
pub mod doudizhu;
pub mod four_player_mahjong;
pub mod guandan;
pub mod mahjong;
pub mod military_chess;
pub mod pao_de_kuai;
pub mod rubiks_cube;
pub mod sheng_ji;
pub mod sudoku;
pub mod texas_holdem;
pub mod two_player_mahjong;

pub use aeroplane_chess::AeroplaneChessRules;
pub use blackjack::BlackjackRules;
pub use bridge::BridgeRules;
pub use chinese_checkers::ChineseCheckersRules;
pub use domino::DominoRules;
pub use doudizhu::DouDiZhuRules;
pub use four_player_mahjong::FourPlayerMahjongRules;
pub use guandan::GuanDanRules;
pub use mahjong::*;
pub use military_chess::MilitaryChessRules;
pub use pao_de_kuai::PaoDeKuaiRules;
pub use rubiks_cube::{CubeType, RubiksCubeRules};
pub use sheng_ji::ShengJiRules;
pub use sudoku::SudokuRules;
pub use texas_holdem::TexasHoldemRules;
pub use two_player_mahjong::TwoPlayerMahjongRules;
