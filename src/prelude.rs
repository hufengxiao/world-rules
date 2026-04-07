//! 预导入模块

pub use crate::rules::{
    Rule, RuleCategory, RuleSet, RuleMetadata, RuleError, RuleResult,
};

// 游戏规则
pub use crate::rules::games::{
    mahjong::{
        MahjongRules, MahjongVariant, SichuanMahjongRules, GuobiaoMahjongRules, RiichiMahjongRules,
        // 国内麻将变体
        GuangdongMahjongRules, WuhanMahjongRules, ShanghaiMahjongRules, TaiwanMahjongRules,
        BeijingMahjongRules, DongbeiMahjongRules, ChangshaMahjongRules, HangzhouMahjongRules,
        NanjingMahjongRules, ChaoshanMahjongRules, TianjinMahjongRules, ChongqingMahjongRules,
        KunmingMahjongRules, GuiyangMahjongRules, FuzhouMahjongRules, NanchangMahjongRules,
        GuangxiMahjongRules, XinjiangMahjongRules, SichuanDetailedMahjongRules,
        ZhengzhouMahjongRules, XianMahjongRules, KejiaMahjongRules, HainanMahjongRules,
        AnhuiMahjongRules, SuzhouMahjongRules,
    },
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
    four_player_mahjong::FourPlayerMahjongRules,
    two_player_mahjong::TwoPlayerMahjongRules,
    domino::DominoRules,
    texas_holdem::TexasHoldemRules,
};

// 棋类规则
pub use crate::rules::games::board_games::{
    ChineseChessRules, ChessRules, GomokuRules, GoRules,
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
    GymnasticsRules, GymnasticsType,
    DivingRules,
    SynchronizedSwimmingRules,
    BoxingRules,
    JudoRules,
    TaekwondoRules,
    BaseballRules,
    IceHockeyRules,
    FigureSkatingRules,
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
    ToastingEtiquette, SeatingEtiquette, GreetingEtiquette,
    PhoneEtiquette, EmailEtiquette,
};

// 科学规则
pub use crate::rules::science::{
    PhysicsLaws, MathRules, ChemistryRules, BiologyRules,
    AstronomyRules, EconomicsRules, PsychologyRules, StatisticsRules,
};

// 法律规则
pub use crate::rules::law::{
    TrafficRules, TrafficRegion, ContractRules, LaborLawRules, ConsumerLawRules,
    IPRules, RoadSafetyRules, MarriageLawRules, InheritanceLawRules,
    CriminalLawRules, CivilLawRules, ConstitutionRules,
};

// 健康规则
pub use crate::rules::health::{
    NutritionRules, ExerciseRules, SleepRules, MentalHealthRules,
};