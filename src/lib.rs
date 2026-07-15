//! # World Rules - 世界规则库
//!
//! 一个收集和提供各种规则的 Rust 库，涵盖游戏、体育、社交、科学、法律、健康六大领域。
//!
//! ## 快速开始
//!
//! ```rust
//! use world_rules::prelude::*;
//!
//! // 验证麻将胡牌
//! let rules = SichuanMahjongRules::new();
//! let result = rules.validate(&ValidateContext::Generic("1万 2万 3万 4万 5万 6万 7万 8万 9万 1条 1条 1条 2条 2条".to_string()));
//! assert!(result.unwrap());
//!
//! // 评估扑克牌型
//! use world_rules::rules::games::card_games::poker::TexasHoldemRules;
//! use world_rules::rules::games::card_games::{Card, Rank, Suit};
//! let cards = vec![
//!     Card::new(Suit::Heart, Rank::Ace),
//!     Card::new(Suit::Heart, Rank::King),
//!     Card::new(Suit::Heart, Rank::Queen),
//!     Card::new(Suit::Heart, Rank::Jack),
//!     Card::new(Suit::Heart, Rank::Ten),
//! ];
//! let eval = TexasHoldemRules::evaluate_hand(&cards);
//! assert_eq!(eval.rank, world_rules::rules::games::card_games::poker::HandRank::RoyalFlush);
//! ```
//!
//! ## 规则分类
//!
//! | 分类 | 数量 | 说明 |
//! |------|------|------|
//! | 🎮 游戏 | 42+ | 麻将、扑克、棋类、桌游 |
//! | 🏃 体育 | 224+ | 球类、格斗、水上、冬季运动 |
//! | 🤝 社交 | 36+ | 餐桌、商务、网络、公共礼仪 |
//! | 🔬 科学 | 132+ | 物理、化学、生物、数学、计算机 |
//! | ⚖️ 法律 | 144+ | 宪法、民法、刑法、国际法 |
//! | 🏥 健康 | 12+ | 营养、运动、睡眠、心理健康 |
//!
//! ## 核心 Trait
//!
//! 所有规则都实现 [`Rule`] trait：
//!
//! - [`Rule::metadata()`] - 获取规则元数据
//! - [`Rule::category()`] - 获取规则分类
//! - [`Rule::validate()`] - 验证状态是否符合规则
//! - [`Rule::explain()`] - 获取规则详细说明
//!
//! ## CLI 工具
//!
//! 启用 `cli` feature 后可使用 `wr` 命令行工具：
//!
//! ```bash
//! cargo build --features cli --bin wr
//! wr list --category sports
//! wr validate mahjong "1万 2万 3万 ..."
//! wr validate poker "Ah Kd Qs Jc 10h"
//! ```

pub mod i18n;
pub mod performance_checker;
pub mod plugins;
pub mod prelude;
pub mod rules;

pub use rules::{Rule, RuleCategory, RuleMetadata, RuleSet, ValidateContext};
pub use performance_checker::{PerformanceBaseline, PerformanceChecker, PerformanceComparison, PerformanceReport};
