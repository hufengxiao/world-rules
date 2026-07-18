//! 游戏设计工具模块
//!
//! 提供游戏设计文档模板、规则平衡性分析、游戏规则验证器和规则复杂度评估等功能。
//!
//! # 示例
//!
//! ```rust
//! use world_rules::rules::game_design_tools::*;
//!
//! // 创建游戏设计文档
//! let template = GameDesignTemplate::new("我的游戏")
//!     .with_genre("策略")
//!     .with_target_audience("青少年");
//!
//! // 生成文档
//! let doc = template.generate_document();
//! assert!(!doc.is_empty());
//! ```

pub mod balance;
pub mod complexity;
pub mod template;
pub mod validator;

// 重新导出主要类型
pub use balance::{BalanceAnalyzer, BalanceReport};
pub use complexity::{ComplexityAnalyzer, ComplexityReport};
pub use template::{GameDesignTemplate, GameDocument};
pub use validator::{GameRuleValidator, ValidationReport};
