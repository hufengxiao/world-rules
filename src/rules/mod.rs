//! 规则模块 - 包含所有规则定义

pub mod analysis;
pub mod core;
pub mod game_design_tools;
pub mod games;
pub mod health;
pub mod law;
pub mod science;
pub mod social;
pub mod sports;

// 重新导出核心类型
pub use core::{
    Difficulty, Rule, RuleCategory, RuleError, RuleMetadata, RuleResult, RuleSet, ValidateContext,
};
