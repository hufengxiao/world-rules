//! 规则模块 - 包含所有规则定义

pub mod core;
pub mod games;
pub mod health;
pub mod law;
pub mod science;
pub mod social;
pub mod sports;

// 重新导出核心类型
pub use core::{Rule, RuleCategory, RuleError, RuleMetadata, RuleResult, RuleSet, ValidateContext};
