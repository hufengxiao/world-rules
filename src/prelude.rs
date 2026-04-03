//! 预导入模块

pub use crate::rules::{
    Rule, RuleCategory, RuleSet, RuleMetadata, RuleError, RuleResult,
};

// 游戏规则
pub use crate::rules::games::{
    mahjong::{MahjongRules, MahjongVariant, SichuanMahjongRules, GuobiaoMahjongRules, RiichiMahjongRules},
};

// 体育规则
pub use crate::rules::sports::{FootballRules, BasketballRules, TableTennisRules};

// 社交礼仪
pub use crate::rules::social::{DiningEtiquette, BusinessEtiquette};

// 科学规则
pub use crate::rules::science::{PhysicsLaws, MathRules};

// 法律规则
pub use crate::rules::law::TrafficRules;