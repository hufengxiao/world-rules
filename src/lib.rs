//! World Rules - 世界规则库
//!
//! 一个收集和提供各种规则的 Rust 库，包括游戏规则、体育规则、社交礼仪等。

pub mod rules;
pub mod prelude;

pub use rules::{Rule, RuleCategory, RuleSet, RuleMetadata};