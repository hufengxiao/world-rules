//! 预导入模块

pub use crate::rules::{Rule, RuleCategory, RuleError, RuleMetadata, RuleResult, RuleSet};

// 游戏规则
pub use crate::rules::games::{
    aeroplane_chess::AeroplaneChessRules,
    blackjack::BlackjackRules,
    bridge::BridgeRules,
    chinese_checkers::ChineseCheckersRules,
    domino::DominoRules,
    doudizhu::DouDiZhuRules,
    four_player_mahjong::FourPlayerMahjongRules,
    guandan::GuanDanRules,
    mahjong::{
        AnhuiMahjongRules,
        BeijingMahjongRules,
        ChangshaMahjongRules,
        ChaoshanMahjongRules,
        ChongqingMahjongRules,
        DongbeiMahjongRules,
        FuzhouMahjongRules,
        // 国内麻将变体
        GuangdongMahjongRules,
        GuangxiMahjongRules,
        GuiyangMahjongRules,
        GuobiaoMahjongRules,
        HainanMahjongRules,
        HangzhouMahjongRules,
        KejiaMahjongRules,
        KunmingMahjongRules,
        MahjongRules,
        MahjongVariant,
        NanchangMahjongRules,
        NanjingMahjongRules,
        RiichiMahjongRules,
        ShanghaiMahjongRules,
        SichuanDetailedMahjongRules,
        SichuanMahjongRules,
        SuzhouMahjongRules,
        TaiwanMahjongRules,
        TianjinMahjongRules,
        WuhanMahjongRules,
        XianMahjongRules,
        XinjiangMahjongRules,
        ZhengzhouMahjongRules,
    },
    military_chess::MilitaryChessRules,
    pao_de_kuai::PaoDeKuaiRules,
    rubiks_cube::{CubeType, RubiksCubeRules},
    sheng_ji::ShengJiRules,
    sudoku::SudokuRules,
    texas_holdem::TexasHoldemRules,
    two_player_mahjong::TwoPlayerMahjongRules,
};

// 棋类规则
pub use crate::rules::games::board_games::{ChessRules, ChineseChessRules, GoRules, GomokuRules};

// 体育规则
pub use crate::rules::sports::{
    ArcheryRules, AthleticsEvent, AthleticsRules, BadmintonRules, BaseballRules, BasketballRules,
    BilliardsRules, BilliardsType, BoxingRules, ClimbingRules, CurlingRules, CyclingRules,
    CyclingType, DivingRules, F1Rules, FencingRules, FigureSkatingRules, FootballRules, GolfRules,
    GymnasticsRules, GymnasticsType, IceHockeyRules, JudoRules, KarateRules, MarathonRules,
    MuayThaiRules, RugbyRules, ShootingRules, SkateboardingRules, SkiingRules, SkiingType,
    SurfingRules, SwimmingRules, SwimmingStyle, SynchronizedSwimmingRules, TableTennisRules,
    TaekwondoRules, TennisRules, TriathlonRules, VolleyballRules, WeightliftingRules,
};

// 社交礼仪
pub use crate::rules::social::{
    BusinessEtiquette, ChineseFestival, CinemaEtiquette, DiningCulture, DiningEtiquette,
    ElevatorEtiquette, EmailEtiquette, FestivalEtiquette, FlightEtiquette, FuneralCulture,
    FuneralEtiquette, GiftEtiquette, GreetingEtiquette, InterviewEtiquette, LibraryEtiquette,
    PhoneEtiquette, SeatingEtiquette, TeaCulture, TeaEtiquette, ToastingEtiquette, TrainEtiquette,
    WeddingCulture, WeddingEtiquette,
};

// 科学规则
pub use crate::rules::science::{
    AstronomyRules, BiologyRules, ChemistryRules, ComputerScienceLaws, EconomicsRules,
    GeoscienceLaws, MaterialScienceLaws, MathRules, NeuroscienceLaws, PhysicsLaws, PsychologyRules,
    QuantumMechanicsLaws, StatisticsRules, ThermodynamicsLaws,
};

// 法律规则
pub use crate::rules::law::{
    AdministrativeLawRules, CivilLawRules, CompanyLawRules, ConstitutionRules, ConsumerLawRules,
    ContractRules, CopyrightLawRules, CriminalLawRules, CybersecurityLawRules, IPRules,
    InheritanceLawRules, LaborLawRules, MarriageLawRules, PatentLawRules, RoadSafetyRules,
    SecuritiesLawRules, TaxLawRules, TrafficRegion, TrafficRules,
};

// 健康规则
pub use crate::rules::health::{ExerciseRules, MentalHealthRules, NutritionRules, SleepRules};
