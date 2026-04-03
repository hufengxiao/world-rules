//! 规则模块 - 包含所有规则定义

pub mod core;
pub mod games;
pub mod sports;
pub mod social;
pub mod science;
pub mod law;

// 重新导出核心类型
pub use core::{Rule, RuleCategory, RuleSet, RuleMetadata, RuleError, RuleResult};