//! # 规则市场 Web 界面
//!
//! 提供规则的浏览、搜索、上传和下载功能。
//!
//! ## 功能特性
//!
//! - 规则浏览：按分类浏览所有可用规则
//! - 规则搜索：根据关键词搜索规则
//! - 规则详情：查看规则的完整信息和使用示例
//! - 规则上传：上传自定义规则包
//! - 规则下载：下载规则到本地使用
//!
//! ## 示例
//!
//! ```rust
//! use world_rules::market::{Marketplace, MarketConfig};
//!
//! // 创建市场实例
//! let config = MarketConfig::default();
//! let market = Marketplace::new(config);
//!
//! // 搜索规则
//! let results = market.search("麻将").unwrap();
//! println!("找到 {} 个规则", results.len());
//! ```

mod search;
mod types;
mod web;

pub use search::*;
pub use types::*;
pub use web::*;
