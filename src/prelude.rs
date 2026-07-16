//! 预导入模块

pub use crate::rules::core::{
    Difficulty, Rule, RuleCategory, RuleError, RuleMetadata, RuleResult, RuleSet, ValidateContext,
};

// 游戏规则 (selective, avoids ambiguous all_rules)
pub use crate::rules::games::{
    backgammon::BackgammonRules,
    blackjack::BlackjackRules,
    board_games::{ChessRules, ChineseChessRules, GoRules, GomokuRules},
    bridge::BridgeRules,
    catan::CatanRules,
    checkers::CheckersRules,
    chess960::Chess960Rules,
    chinese_checkers::ChineseCheckersRules,
    codenames::CodenamesRules,
    connect_four::ConnectFourRules,
    craps::CrapsRules,
    domino::DominoRules,
    domino_detailed::DominoDetailedRules,
    doudizhu::DouDiZhuRules,
    four_player_mahjong::FourPlayerMahjongRules,
    gongzhu::GongzhuRules,
    guandan::GuanDanRules,
    hearts::HeartsRules,
    mafia::MafiaRules,
    mahjong::*,
    mahjong_riichi_detailed::MahjongRiichiDetailedRules,
    military_chess::MilitaryChessRules,
    monopoly::MonopolyRules,
    niuniu::NiuniuRules,
    pao_de_kuai::PaoDeKuaiRules,
    pictionary::PictionaryRules,
    reversi::ReversiRules,
    risk::RiskRules,
    rubiks_cube::{CubeType, RubiksCubeRules},
    scrabble::ScrabbleRules,
    sheng_ji::ShengJiRules,
    stud_poker::StudPokerRules,
    sudoku::SudokuRules,
    sudoku_variant::SudokuVariantRules,
    texas_holdem::TexasHoldemRules,
    texas_holdem_detailed::TexasHoldemDetailedRules,
    twenty_four_point::TwentyFourPointRules,
    two_player_mahjong::TwoPlayerMahjongRules,
    uno::UnoRules,
    werewolf::WerewolfRules,
    who_is_spy::WhoIsSpyRules,
    yahtzee::YahtzeeRules,
    zhajinhua::ZhajinhuaRules,
};

// 体育规则
pub use crate::rules::sports::{
    archery_detailed::ArcheryDetailedRules, athletics_detailed::AthleticsDetailedRules,
    badminton::BadmintonRules, badminton_detailed::BadmintonDetailedRules, baseball::BaseballRules,
    baseball_detailed::BaseballDetailedRules, basketball::BasketballRules,
    basketball_detailed::BasketballDetailedRules, billiards::BilliardsRules, boxing::BoxingRules,
    boxing_detailed::BoxingDetailedRules, climbing::ClimbingRules, curling::CurlingRules,
    diving::DivingRules, f1::F1Rules, fencing::FencingRules, figure_skating::FigureSkatingRules,
    football::FootballRules, football_detailed::FootballDetailedRules, golf::GolfRules,
    ice_hockey::IceHockeyRules, judo::JudoRules, marathon::MarathonRules, muay_thai::MuayThaiRules,
    rugby::RugbyRules, shooting::ShootingRules, skateboarding::SkateboardingRules,
    skiing::SkiingRules, surfing::SurfingRules, swimming::SwimmingRules,
    table_tennis::TableTennisRules, taekwondo::TaekwondoRules, tennis::TennisRules,
    triathlon::TriathlonRules, volleyball::VolleyballRules, weightlifting::WeightliftingRules,
};

// 社交礼仪
pub use crate::rules::social::{
    business::BusinessEtiquette,
    cinema::CinemaEtiquette,
    dining::{DiningCulture, DiningEtiquette},
    elevator::ElevatorEtiquette,
    email::EmailEtiquette,
    festival::{ChineseFestival, FestivalEtiquette},
    flight::FlightEtiquette,
    funeral::{FuneralCulture, FuneralEtiquette},
    gift::GiftEtiquette,
    greeting::GreetingEtiquette,
    interview::InterviewEtiquette,
    library::LibraryEtiquette,
    phone::PhoneEtiquette,
    seating::SeatingEtiquette,
    tea::{TeaCulture, TeaEtiquette},
    toasting::ToastingEtiquette,
    train::TrainEtiquette,
    wedding::{WeddingCulture, WeddingEtiquette},
};

// 科学规则 (selective, avoids name conflicts with law)
pub use crate::rules::science::{
    astronomy::AstronomyRules, biology::BiologyRules, chemistry::ChemistryRules,
    economics::EconomicsRules, math::MathRules, physics::PhysicsLaws, psychology::PsychologyRules,
    statistics::StatisticsRules,
};

// 法律规则
pub use crate::rules::law::{
    administrative::AdministrativeLawRules,
    civil::CivilLawRules,
    company::CompanyLawRules,
    constitution::ConstitutionRules,
    consumer::ConsumerLawRules,
    contract::ContractRules,
    copyright::CopyrightLawRules,
    criminal::CriminalLawRules,
    cybersecurity::CybersecurityLawRules,
    inheritance::InheritanceLawRules,
    intellectual_property::IPRules,
    labor::LaborLawRules,
    marriage::MarriageLawRules,
    patent::PatentLawRules,
    road_safety::RoadSafetyRules,
    securities::SecuritiesLawRules,
    tax::TaxLawRules,
    traffic::{TrafficRegion, TrafficRules},
};

// 健康规则
pub use crate::rules::health::{
    exercise::ExerciseRules, mental_health::MentalHealthRules, nutrition::NutritionRules,
    sleep::SleepRules,
};
