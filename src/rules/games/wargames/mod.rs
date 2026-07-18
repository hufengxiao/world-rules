//! 战争游戏（War Games）规则模块
//!
//! 本模块实现各类桌面战争游戏和微缩模型游戏的规则，包括：
//! - **战锤40K**：Warhammer 40,000 科幻战争游戏
//! - **战锤: 西格玛时代**：Warhammer Age of Sigmar 奇幻战争游戏
//! - **通用战棋规则**：回合制战棋游戏基础规则
//! - **微缩模型游戏**：通用微缩模型游戏框架
//!
//! # 战争游戏特点
//!
//! 战争游戏的核心是：
//! - 军队构建（点数系统、单位选择）
//! - 战术部署（地形、阵型）
//! - 回合制战斗（移动、射击、近战）
//! - 特殊规则（指挥、士气、能力）
//!
//! # Examples
//!
//! ```rust
//! use world_rules::rules::games::wargames::{Warhammer40KRules, AgeOfSigmarRules};
//! use world_rules::rules::core::Rule;
//!
//! // 战锤40K 规则
//! let wh40k = Warhammer40KRules::new();
//! let phases = wh40k.game_phases();
//! assert_eq!(phases.len(), 5);
//!
//! // 西格玛时代规则
//! let aos = AgeOfSigmarRules::new();
//! assert!(aos.point_limit_default() > 0);
//! ```

pub mod age_of_sigmar;
pub mod generic_wargame;
pub mod miniature_games;
pub mod warhammer_40k;

// 重新导出主要类型
pub use age_of_sigmar::AgeOfSigmarRules;
pub use generic_wargame::GenericWargameRules;
pub use miniature_games::MiniatureGameRules;
pub use warhammer_40k::Warhammer40KRules;