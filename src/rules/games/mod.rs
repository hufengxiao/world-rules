//! 游戏规则模块 - 涵盖各类游戏的规则实现
//!
//! 本模块包含 100+ 种游戏的完整规则，包括：
//! - **棋类游戏**: 国际象棋、中国象棋、围棋、五子棋等
//! - **卡牌游戏**: 扑克、斗地主、麻将、德州扑克等
//! - **桌游**: 大富翁、卡坦岛、风险等
//! - **益智游戏**: 数独、魔方、24 点等
//!
//! # 模块结构
//!
//! ```text
//! games/
//! ├── board_games/    # 棋类游戏（象棋、围棋等）
//! ├── card_games/     # 卡牌游戏（扑克牌型评估）
//! ├── mahjong/        # 麻将核心逻辑和各地变体
//! ├── *其他游戏       # 各类独立游戏规则
//! ```
//!
//! # Examples
//!
//! 使用规则示例：
//!
//! ```rust
//! use world_rules::rules::games::{ChessRules, DouDiZhuRules};
//! use world_rules::rules::core::Rule;
//!
//! // 国际象棋规则
//! let chess = ChessRules::new();
//! println!("规则: {}", chess.metadata().name);
//! println!("分类: {:?}", chess.category());
//!
//! // 斗地主规则
//! let ddz = DouDiZhuRules::new();
//! let explanation = ddz.explain();
//! assert!(!explanation.is_empty());
//! ```
//!
//! # 子模块
//!
//! - [`board_games`] - 棋类和棋盘游戏
//! - [`card_games`] - 卡牌游戏和扑克牌型评估
//! - [`mahjong`] - 麻将游戏及各地变体（四川、日本、台湾等）
//! - [`poker_chinese`] - 中国扑克规则
//! - [`poker_five_card`] - 五张牌扑克规则
//! - [`poker_omaha`] - 奥马哈扑克规则
//! - [`texas_holdem`] - 德州扑克规则
//!
//! # 规则统计
//!
//! 当前包含 1098 条游戏规则，覆盖：
//! - 40+ 种麻将变体
//! - 20+ 种扑克变体
//! - 15+ 种棋类游戏
//! - 30+ 种桌游
//! - 20+ 种其他游戏

pub mod abalone;
pub mod aeroplane_chess;
pub mod agricola;
pub mod azul;
pub mod baccarat;
pub mod backgammon;
pub mod big_two;
pub mod blackjack;
pub mod blind_chess;
pub mod blind_go;
pub mod board_games;
pub mod bridge;
pub mod bridge_chicago;
pub mod bridge_detailed;
pub mod bridge_duplicate;
pub mod bridge_imp;
pub mod bridge_minibridge;
pub mod bridge_rubber;
pub mod canasta;
pub mod canfield;
pub mod carcassonne;
pub mod card_game_generic;
pub mod card_games;
pub mod caribbean_stud;
pub mod cassino;
pub mod catan;
pub mod chaturanga;
pub mod checkers;
pub mod chess960;
pub mod chess960_detailed;
pub mod chess_detailed;
pub mod chinese_checkers;
pub mod codenames;
pub mod connect_four;
pub mod craps;
pub mod crazy_eights;
pub mod cribbage;
pub mod dark_chess;
pub mod dice_game;
pub mod dominion;
pub mod domino;
pub mod domino_detailed;
pub mod dots_and_boxes;
pub mod doudizhu;
pub mod euchre;
pub mod euchre_detailed;
pub mod four_player_chess;
pub mod four_player_mahjong;
pub mod gin_rummy;
pub mod go_13x13;
pub mod go_9x9;
pub mod go_detailed;
pub mod go_fish;
pub mod gomoku_detailed;
pub mod gongzhu;
pub mod guandan;
pub mod hearts;
pub mod hearts_detailed;
pub mod hive;
pub mod janggi;
pub mod jungle;
pub mod klondike_solitaire;
pub mod mafia;
pub mod mahjong;
pub mod mahjong_blood_battle;
pub mod mahjong_cantonese_detailed;
pub mod mahjong_changsha_detailed;
pub mod mahjong_filipino;
pub mod mahjong_hangzhou_detailed;
pub mod mahjong_hongkong;
pub mod mahjong_japanese_detailed;
pub mod mahjong_korean;
pub mod mahjong_malaysian;
pub mod mahjong_riichi_detailed;
pub mod mahjong_riichi_detailed2;
pub mod mahjong_sichuan_detailed;
pub mod mahjong_singapore;
pub mod mahjong_taiwanese_detailed;
pub mod mahjong_vietnamese;
pub mod mahjong_wuhan_detailed;
pub mod makruk;
pub mod mancala;
pub mod military_chess;
pub mod mini_chess;
pub mod mini_shogi;
pub mod monopoly;
pub mod mystery_card;
pub mod niuniu;
pub mod old_maid;
pub mod one_color_go;
pub mod othello_detailed2;
pub mod pai_gow_poker;
pub mod pair_go;
pub mod pandemic;
pub mod pao_de_kuai;
pub mod party_game;
pub mod pictionary;
pub mod pinochle;
pub mod poker_chinese;
pub mod poker_five_card;
pub mod poker_omaha;
pub mod power_grid;
pub mod puerto_rico;
pub mod pyramid_solitaire;
pub mod quarto;
pub mod reversi;
pub mod risk;
pub mod rubiks_cube;
pub mod rummy_detailed;
pub mod scrabble;
pub mod seven_wonders;
pub mod sheng_ji;
pub mod shogi;
pub mod snap;
pub mod spades;
pub mod speed_card;
pub mod splendor;
pub mod stratego;
pub mod stud_poker;
pub mod sudoku;
pub mod sudoku_variant;
pub mod tak;
pub mod tarot_cards;
pub mod texas_holdem;
pub mod texas_holdem_detailed;
pub mod three_card_poker;
pub mod tic_tac_toe;
pub mod ticket_to_ride;
pub mod trivia_game;
pub mod twenty_four_point;
pub mod two_player_mahjong;
pub mod uno;
pub mod war_card;
pub mod werewolf;
pub mod who_is_spy;
pub mod word_game;
pub mod xiangqi960;
pub mod xiangqi_detailed;
pub mod yahtzee;
pub mod zhajinhua;

pub use abalone::AbaloneRules;
pub use aeroplane_chess::AeroplaneChessRules;
pub use agricola::AgricolaRules;
pub use azul::AzulRules;
pub use baccarat::BaccaratRules;
pub use backgammon::BackgammonRules;
pub use big_two::BigTwoRules;
pub use blackjack::BlackjackRules;
pub use blind_chess::BlindChessRules;
pub use blind_go::BlindGoRules;
pub use board_games::{ChessRules, ChineseChessRules, GoRules, GomokuRules};
pub use bridge::BridgeRules;
pub use bridge_chicago::BridgeChicagoRules;
pub use bridge_detailed::BridgeDetailedRules;
pub use bridge_duplicate::BridgeDuplicateRules;
pub use bridge_imp::BridgeImpRules;
pub use bridge_minibridge::BridgeMinibridgeRules;
pub use bridge_rubber::BridgeRubberRules;
pub use canasta::CanastaRules;
pub use canfield::CanfieldRules;
pub use carcassonne::CarcassonneRules;
pub use card_game_generic::CardGameGenericRules;
pub use caribbean_stud::CaribbeanStudRules;
pub use cassino::CassinoRules;
pub use catan::CatanRules;
pub use chaturanga::ChaturangaRules;
pub use checkers::CheckersRules;
pub use chess960::Chess960Rules;
pub use chess960_detailed::Chess960DetailedRules;
pub use chess_detailed::ChessDetailedRules;
pub use chinese_checkers::ChineseCheckersRules;
pub use codenames::CodenamesRules;
pub use connect_four::ConnectFourRules;
pub use craps::CrapsRules;
pub use crazy_eights::CrazyEightsRules;
pub use cribbage::CribbageRules;
pub use dark_chess::DarkChessRules;
pub use dice_game::DiceGameRules;
pub use dominion::DominionRules;
pub use domino::DominoRules;
pub use domino_detailed::DominoDetailedRules;
pub use dots_and_boxes::DotsAndBoxesRules;
pub use doudizhu::DouDiZhuRules;
pub use euchre::EuchreRules;
pub use euchre_detailed::EuchreDetailedRules;
pub use four_player_chess::FourPlayerChessRules;
pub use four_player_mahjong::FourPlayerMahjongRules;
pub use gin_rummy::GinRummyRules;
pub use go_13x13::Go13x13Rules;
pub use go_9x9::Go9x9Rules;
pub use go_detailed::GoDetailedRules;
pub use go_fish::GoFishRules;
pub use gomoku_detailed::GomokuDetailedRules;
pub use gongzhu::GongzhuRules;
pub use guandan::GuanDanRules;
pub use hearts::HeartsRules;
pub use hearts_detailed::HeartsDetailedRules;
pub use hive::HiveRules;
pub use janggi::JanggiRules;
pub use jungle::JungleRules;
pub use klondike_solitaire::KlondikeSolitaireRules;
pub use mafia::MafiaRules;
pub use mahjong::*;
pub use mahjong_blood_battle::MahjongBloodBattleRules;
pub use mahjong_cantonese_detailed::MahjongCantoneseDetailedRules;
pub use mahjong_changsha_detailed::MahjongChangshaDetailedRules;
pub use mahjong_filipino::MahjongFilipinoRules;
pub use mahjong_hangzhou_detailed::MahjongHangzhouDetailedRules;
pub use mahjong_hongkong::MahjongHongkongRules;
pub use mahjong_japanese_detailed::MahjongJapaneseDetailedRules;
pub use mahjong_korean::MahjongKoreanRules;
pub use mahjong_malaysian::MahjongMalaysianRules;
pub use mahjong_riichi_detailed::MahjongRiichiDetailedRules;
pub use mahjong_riichi_detailed2::MahjongRiichiDetailed2Rules;
pub use mahjong_sichuan_detailed::MahjongSichuanDetailedRules;
pub use mahjong_singapore::MahjongSingaporeRules;
pub use mahjong_taiwanese_detailed::MahjongTaiwaneseDetailedRules;
pub use mahjong_vietnamese::MahjongVietnameseRules;
pub use mahjong_wuhan_detailed::MahjongWuhanDetailedRules;
pub use makruk::MakrukRules;
pub use mancala::MancalaRules;
pub use military_chess::MilitaryChessRules;
pub use mini_chess::MiniChessRules;
pub use mini_shogi::MiniShogiRules;
pub use monopoly::MonopolyRules;
pub use mystery_card::MysteryCardRules;
pub use niuniu::NiuniuRules;
pub use old_maid::OldMaidRules;
pub use one_color_go::OneColorGoRules;
pub use othello_detailed2::OthelloDetailed2Rules;
pub use pai_gow_poker::PaiGowPokerRules;
pub use pair_go::PairGoRules;
pub use pandemic::PandemicRules;
pub use pao_de_kuai::PaoDeKuaiRules;
pub use party_game::PartyGameRules;
pub use pictionary::PictionaryRules;
pub use pinochle::PinochleRules;
pub use poker_chinese::PokerChineseRules;
pub use poker_five_card::PokerFiveCardRules;
pub use poker_omaha::PokerOmahaRules;
pub use power_grid::PowerGridRules;
pub use puerto_rico::PuertoRicoRules;
pub use pyramid_solitaire::PyramidSolitaireRules;
pub use quarto::QuartoRules;
pub use reversi::ReversiRules;
pub use risk::RiskRules;
pub use rubiks_cube::{CubeType, RubiksCubeRules};
pub use rummy_detailed::RummyDetailedRules;
pub use scrabble::ScrabbleRules;
pub use seven_wonders::SevenWondersRules;
pub use sheng_ji::ShengJiRules;
pub use shogi::ShogiRules;
pub use snap::SnapRules;
pub use spades::SpadesRules;
pub use speed_card::SpeedCardRules;
pub use splendor::SplendorRules;
pub use stratego::StrategoRules;
pub use stud_poker::StudPokerRules;
pub use sudoku::SudokuRules;
pub use sudoku_variant::SudokuVariantRules;
pub use tak::TakRules;
pub use tarot_cards::TarotCardsRules;
pub use texas_holdem::TexasHoldemRules;
pub use texas_holdem_detailed::TexasHoldemDetailedRules;
pub use three_card_poker::ThreeCardPokerRules;
pub use tic_tac_toe::TicTacToeRules;
pub use ticket_to_ride::TicketToRideRules;
pub use trivia_game::TriviaGameRules;
pub use twenty_four_point::TwentyFourPointRules;
pub use two_player_mahjong::TwoPlayerMahjongRules;
pub use uno::UnoRules;
pub use war_card::WarCardRules;
pub use werewolf::WerewolfRules;
pub use who_is_spy::WhoIsSpyRules;
pub use word_game::WordGameRules;
pub use xiangqi960::Xiangqi960Rules;
pub use xiangqi_detailed::XiangqiDetailedRules;
pub use yahtzee::YahtzeeRules;
pub use zhajinhua::ZhajinhuaRules;

/// 获取所有游戏规则的元数据列表
///
/// 返回一个包含所有游戏规则的元数据、分类和说明的向量。
/// 用于遍历和展示所有规则。
///
/// # Returns
///
/// 返回一个 Vec，每个元素是一个元组：
/// - `&'static str`: 规则分类标识（如 "games"）
/// - `RuleMetadata`: 规则元数据
/// - `RuleCategory`: 规则分类
/// - `String`: 规则说明文本
///
/// # Examples
/// ```
/// use world_rules::rules::games::all_rules;
///
/// let rules = all_rules();
/// assert!(!rules.is_empty());
/// // 每个元素包含分类标识、元数据、分类和说明
/// for (category, meta, cat, explain) in &rules {
///     println!("{}: {}", meta.name, category);
/// }
/// ```
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
        let r = AgricolaRules::new();
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
        let r = BaccaratRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BigTwoRules::new();
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
        let r = BridgeChicagoRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BridgeDuplicateRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BridgeImpRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BridgeMinibridgeRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BridgeRubberRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CanfieldRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CarcassonneRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CassinoRules::new();
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
        let r = DominionRules::new();
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
        let r = GinRummyRules::new();
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
        let r = KlondikeSolitaireRules::new();
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
        let r = PaiGowPokerRules::new();
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
        let r = PowerGridRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PuertoRicoRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PyramidSolitaireRules::new();
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
        let r = ThreeCardPokerRules::new();
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
    {
        let r = JanggiRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = JungleRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MakrukRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MancalaRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MiniShogiRules::new();
        rules.push(("games", r.metadata().clone(), r.category(), r.explain()));
    }
    rules
}
