//! 角色扮演游戏（RPG）规则模块
//!
//! 本模块实现各类桌面角色扮演游戏的规则，包括：
//! - **D&D 系列**：龙与地下城基础规则和第五版核心规则
//! - **Pathfinder**：路径探路者第一版和第二版
//! - **Call of Cthulhu**：克苏鲁的呼唤恐怖游戏
//!
//! # RPG 规则特点
//!
//! 桌面 RPG 的核心是：
//! - 规则系统（属性、技能、检定）
//! - 角色创建（种族、职业、背景）
//! - 战斗系统（回合制、行动经济）
//! - 故事叙述（由 Game Master 主持）
//!
//! # Examples
//!
//! ```rust
//! use world_rules::rules::games::rpg::{DndBasicRules, PathfinderRules};
//! use world_rules::rules::core::Rule;
//!
//! // D&D 基础规则
//! let dnd = DndBasicRules::new();
//! let abilities = dnd.section_ability_scores();
//! assert!(abilities.len() >= 6);
//!
//! // Pathfinder 规则
//! let pf = PathfinderRules::new();
//! let classes = pf.section_pf1_classes();
//! assert_eq!(classes.len(), 12);
//! ```

pub mod call_of_cthulhu;
pub mod dnd_5e_core;
pub mod dnd_basic;
pub mod pathfinder;

// 重新导出主要类型
pub use call_of_cthulhu::CallOfCthulhuRules;
pub use dnd_5e_core::Dnd5eCoreRules;
pub use dnd_basic::DndBasicRules;
pub use pathfinder::PathfinderRules;
