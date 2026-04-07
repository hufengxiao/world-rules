//! 预导入模块

pub use crate::rules::{
    Rule, RuleCategory, RuleSet, RuleMetadata, RuleError, RuleResult,
};

// 游戏规则
pub use crate::rules::games::{
    mahjong::{MahjongRules, MahjongVariant, SichuanMahjongRules, GuobiaoMahjongRules, RiichiMahjongRules},
    doudizhu::DouDiZhuRules,
    blackjack::BlackjackRules,
    bridge::BridgeRules,
    sudoku::SudokuRules,
    rubiks_cube::{RubiksCubeRules, CubeType},
    pao_de_kuai::PaoDeKuaiRules,
    sheng_ji::ShengJiRules,
    aeroplane_chess::AeroplaneChessRules,
    chinese_checkers::ChineseCheckersRules,
    guandan::GuanDanRules,
    military_chess::MilitaryChessRules,
};

// 棋类规则
pub use crate::rules::games::board_games::{
    ChineseChessRules, ChessRules, GomokuRules,
};

// 体育规则
pub use crate::rules::sports::{
    FootballRules, BasketballRules, TableTennisRules,
    TennisRules, VolleyballRules, BadmintonRules,
    SwimmingRules, SwimmingStyle,
    AthleticsRules, AthleticsEvent,
    GolfRules,
    SkiingRules, SkiingType,
    CyclingRules, CyclingType,
    WeightliftingRules,
    BilliardsRules, BilliardsType,
    ShootingRules,
    ArcheryRules,
};

// 社交礼仪
pub use crate::rules::social::{
    DiningEtiquette, DiningCulture, BusinessEtiquette,
    GiftEtiquette, TeaEtiquette, TeaCulture,
    WeddingEtiquette, WeddingCulture, InterviewEtiquette,
    FuneralEtiquette, FuneralCulture,
    FestivalEtiquette, ChineseFestival,
    ElevatorEtiquette, CinemaEtiquette, LibraryEtiquette,
    FlightEtiquette, TrainEtiquette,
};

// 科学规则
pub use crate::rules::science::{
    PhysicsLaws, MathRules, ChemistryRules, BiologyRules,
    AstronomyRules, EconomicsRules,
};

// 法律规则
pub use crate::rules::law::{
    TrafficRules, TrafficRegion, ContractRules, LaborLawRules, ConsumerLawRules,
    IPRules, RoadSafetyRules,
};