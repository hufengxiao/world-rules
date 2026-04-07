//! 预导入模块

pub use crate::rules::{
    Rule, RuleCategory, RuleSet, RuleMetadata, RuleError, RuleResult,
};

// 游戏规则
pub use crate::rules::games::{
    mahjong::{MahjongRules, MahjongVariant, SichuanMahjongRules, GuobiaoMahjongRules, RiichiMahjongRules},
    doudizhu::DouDiZhuRules,
    blackjack::BlackjackRules,
};

// 棋类规则
pub use crate::rules::games::board_games::{
    ChineseChessRules, ChessRules, GomokuRules,
};

// 体育规则
pub use crate::rules::sports::{
    FootballRules, BasketballRules, TableTennisRules,
    TennisRules, VolleyballRules, BadmintonRules,
};

// 社交礼仪
pub use crate::rules::social::{
    DiningEtiquette, DiningCulture, BusinessEtiquette,
    GiftEtiquette, TeaEtiquette, TeaCulture,
    WeddingEtiquette, WeddingCulture, InterviewEtiquette,
};

// 科学规则
pub use crate::rules::science::{
    PhysicsLaws, MathRules, ChemistryRules,
};

// 法律规则
pub use crate::rules::law::{
    TrafficRules, TrafficRegion, ContractRules, LaborLawRules,
};