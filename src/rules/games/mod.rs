//! 游戏规则模块

pub mod aeroplane_chess;
pub mod backgammon;
pub mod blackjack;
pub mod board_games;
pub mod bridge;
pub mod card_games;
pub mod catan;
pub mod checkers;
pub mod chess960;
pub mod chinese_checkers;
pub mod codenames;
pub mod connect_four;
pub mod craps;
pub mod domino;
pub mod domino_detailed;
pub mod doudizhu;
pub mod four_player_mahjong;
pub mod gongzhu;
pub mod guandan;
pub mod hearts;
pub mod mafia;
pub mod mahjong;
pub mod mahjong_riichi_detailed;
pub mod military_chess;
pub mod monopoly;
pub mod niuniu;
pub mod pao_de_kuai;
pub mod pictionary;
pub mod reversi;
pub mod risk;
pub mod rubiks_cube;
pub mod scrabble;
pub mod sheng_ji;
pub mod stud_poker;
pub mod sudoku;
pub mod sudoku_variant;
pub mod texas_holdem;
pub mod texas_holdem_detailed;
pub mod twenty_four_point;
pub mod two_player_mahjong;
pub mod uno;
pub mod werewolf;
pub mod who_is_spy;
pub mod yahtzee;
pub mod zhajinhua;

pub use aeroplane_chess::AeroplaneChessRules;
pub use backgammon::BackgammonRules;
pub use blackjack::BlackjackRules;
pub use board_games::{ChessRules, ChineseChessRules, GoRules, GomokuRules};
pub use bridge::BridgeRules;
pub use catan::CatanRules;
pub use checkers::CheckersRules;
pub use chess960::Chess960Rules;
pub use chinese_checkers::ChineseCheckersRules;
pub use codenames::CodenamesRules;
pub use connect_four::ConnectFourRules;
pub use craps::CrapsRules;
pub use domino::DominoRules;
pub use domino_detailed::DominoDetailedRules;
pub use doudizhu::DouDiZhuRules;
pub use four_player_mahjong::FourPlayerMahjongRules;
pub use gongzhu::GongzhuRules;
pub use guandan::GuanDanRules;
pub use hearts::HeartsRules;
pub use mafia::MafiaRules;
pub use mahjong::*;
pub use mahjong_riichi_detailed::MahjongRiichiDetailedRules;
pub use military_chess::MilitaryChessRules;
pub use monopoly::MonopolyRules;
pub use niuniu::NiuniuRules;
pub use pao_de_kuai::PaoDeKuaiRules;
pub use pictionary::PictionaryRules;
pub use reversi::ReversiRules;
pub use risk::RiskRules;
pub use rubiks_cube::{CubeType, RubiksCubeRules};
pub use scrabble::ScrabbleRules;
pub use sheng_ji::ShengJiRules;
pub use stud_poker::StudPokerRules;
pub use sudoku::SudokuRules;
pub use sudoku_variant::SudokuVariantRules;
pub use texas_holdem::TexasHoldemRules;
pub use texas_holdem_detailed::TexasHoldemDetailedRules;
pub use twenty_four_point::TwentyFourPointRules;
pub use two_player_mahjong::TwoPlayerMahjongRules;
pub use uno::UnoRules;
pub use werewolf::WerewolfRules;
pub use who_is_spy::WhoIsSpyRules;
pub use yahtzee::YahtzeeRules;
pub use zhajinhua::ZhajinhuaRules;

pub fn all_rules() -> Vec<(
    &'static str,
    crate::rules::core::RuleMetadata,
    crate::rules::core::RuleCategory,
    String,
)> {
    use crate::rules::core::Rule;
    let mut rules = Vec::new();
    {
        let r = AeroplaneChessRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AnhuiMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BackgammonRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BeijingMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BlackjackRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BridgeRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CatanRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChangshaMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChaoshanMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CheckersRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = Chess960Rules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChessRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChineseCheckersRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChineseChessRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChongqingMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CodenamesRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ConnectFourRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CrapsRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DominoDetailedRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DominoRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DongbeiMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DouDiZhuRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FourPlayerMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FuzhouMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GoRules::new(19);
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GomokuRules::new(crate::rules::games::board_games::gomoku::GomokuVariant::Standard);
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GongzhuRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GuanDanRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GuangdongMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GuangxiMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GuiyangMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GuobiaoMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HainanMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HangzhouMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HeartsRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = KejiaMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = KunmingMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MafiaRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MahjongRiichiDetailedRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MonopolyRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MilitaryChessRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NanchangMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NanjingMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NiuniuRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PaoDeKuaiRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PictionaryRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ReversiRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RiichiMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RiskRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RubiksCubeRules::new(crate::rules::games::rubiks_cube::CubeType::ThreeByThree);
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ScrabbleRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ShanghaiMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ShengJiRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SichuanDetailedMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SichuanMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = StudPokerRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SudokuRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SudokuVariantRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SuzhouMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TaiwanMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TexasHoldemDetailedRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TexasHoldemRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TianjinMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TwentyFourPointRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TwoPlayerMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = UnoRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WerewolfRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WhoIsSpyRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WuhanMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = XianMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = XinjiangMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = YahtzeeRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ZhajinhuaRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ZhengzhouMahjongRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    rules
}
