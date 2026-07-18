//! 规则智能分析模块
//!
//! 提供规则冲突检测、完整性分析、复杂度分析和质量评分功能。
//!
//! # 功能特性
//!
//! - **规则冲突检测**: 检测规则集合中的矛盾和冲突
//! - **规则完整性分析**: 分析规则是否定义完整
//! - **规则复杂度分析**: 评估规则的复杂度级别
//! - **规则质量评分**: 综合评估规则质量
//!
//! # 示例
//!
//! ```rust
//! use world_rules::rules::analysis::*;
//! use world_rules::rules::core::{RuleMetadata, RuleCategory, Rule};
//!
//! // 创建冲突检测器
//! let detector = ConflictDetector::new();
//!
//! // 创建完整性分析器
//! let completeness_analyzer = CompletenessAnalyzer::new();
//!
//! // 创建复杂度分析器
//! let complexity_analyzer = RuleComplexityAnalyzer::new();
//!
//! // 创建质量评分器
//! let quality_scorer = QualityScorer::new();
//! ```

pub mod completeness;
pub mod complexity;
pub mod conflict;
pub mod quality;

pub use completeness::{CompletenessAnalyzer, CompletenessLevel, CompletenessReport};
pub use complexity::{ComplexityLevel, RuleComplexityAnalyzer, RuleComplexityReport};
pub use conflict::{ConflictDetector, ConflictReport, ConflictSeverity, ConflictType};
pub use quality::{QualityDimension, QualityReport, QualityScorer};
