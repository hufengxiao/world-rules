//! # 规则市场类型定义
//!
//! 定义规则市场所需的核心数据结构。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 规则市场配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketConfig {
    /// 市场名称
    pub name: String,
    /// API 端点
    pub api_endpoint: String,
    /// 是否启用缓存
    pub enable_cache: bool,
    /// 缓存过期时间（秒）
    pub cache_ttl_seconds: u64,
    /// 每页显示数量
    pub page_size: usize,
}

impl Default for MarketConfig {
    fn default() -> Self {
        Self {
            name: "World Rules Marketplace".to_string(),
            api_endpoint: "https://api.worldrules.io/v1".to_string(),
            enable_cache: true,
            cache_ttl_seconds: 3600,
            page_size: 20,
        }
    }
}

/// 规则市场实例
#[derive(Debug)]
pub struct Marketplace {
    /// 市场配置
    #[allow(dead_code)]
    config: MarketConfig,
    /// 规则缓存
    rule_cache: HashMap<String, RulePackage>,
    /// 搜索索引
    search_index: HashMap<String, Vec<String>>,
}

impl Marketplace {
    /// 创建新的市场实例
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::market::{Marketplace, MarketConfig};
    ///
    /// let config = MarketConfig::default();
    /// let market = Marketplace::new(config);
    /// ```
    pub fn new(config: MarketConfig) -> Self {
        Self {
            config,
            rule_cache: HashMap::new(),
            search_index: HashMap::new(),
        }
    }

    /// 搜索规则
    ///
    /// # Arguments
    ///
    /// * `query` - 搜索关键词
    ///
    /// # Returns
    ///
    /// 返回匹配的规则列表
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::market::{Marketplace, MarketConfig};
    ///
    /// let market = Marketplace::new(MarketConfig::default());
    /// let results = market.search("麻将").unwrap();
    /// ```
    pub fn search(&self, query: &str) -> Result<Vec<RulePackage>, MarketError> {
        let mut results = Vec::new();

        // 从缓存中搜索
        for rule in self.rule_cache.values() {
            if rule.matches(query) {
                results.push(rule.clone());
            }
        }

        // 按评分排序
        results.sort_by(|a, b| {
            b.rating
                .partial_cmp(&a.rating)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(results)
    }

    /// 获取规则详情
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 规则ID
    pub fn get_rule(&self, rule_id: &str) -> Option<&RulePackage> {
        self.rule_cache.get(rule_id)
    }

    /// 上传规则包
    ///
    /// # Arguments
    ///
    /// * `package` - 规则包
    pub fn upload(&mut self, package: RulePackage) -> Result<String, MarketError> {
        let rule_id = package.id.clone();

        // 更新搜索索引
        for tag in &package.tags {
            self.search_index
                .entry(tag.clone())
                .or_default()
                .push(rule_id.clone());
        }

        // 添加到缓存
        self.rule_cache.insert(rule_id.clone(), package);

        Ok(rule_id)
    }

    /// 下载规则包
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 规则ID
    pub fn download(&self, rule_id: &str) -> Option<&RulePackage> {
        self.rule_cache.get(rule_id)
    }

    /// 获取市场统计信息
    pub fn stats(&self) -> MarketStats {
        MarketStats {
            total_rules: self.rule_cache.len(),
            total_downloads: self.rule_cache.values().map(|r| r.downloads).sum(),
            categories: self
                .rule_cache
                .values()
                .map(|r| r.category.clone())
                .collect(),
        }
    }
}

/// 规则包
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulePackage {
    /// 规则ID
    pub id: String,
    /// 规则名称
    pub name: String,
    /// 规则描述
    pub description: String,
    /// 规则分类
    pub category: String,
    /// 作者
    pub author: String,
    /// 版本
    pub version: String,
    /// 评分（0-5）
    pub rating: f32,
    /// 下载次数
    pub downloads: u64,
    /// 标签
    pub tags: Vec<String>,
    /// 规则内容（JSON格式）
    pub content: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl RulePackage {
    /// 创建新的规则包
    pub fn new(id: String, name: String, category: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            name,
            description: String::new(),
            category,
            author: String::new(),
            version: "1.0.0".to_string(),
            rating: 0.0,
            downloads: 0,
            tags: Vec::new(),
            content: String::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// 检查是否匹配搜索查询
    pub fn matches(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();

        // 检查名称
        if self.name.to_lowercase().contains(&query_lower) {
            return true;
        }

        // 检查描述
        if self.description.to_lowercase().contains(&query_lower) {
            return true;
        }

        // 检查标签
        for tag in &self.tags {
            if tag.to_lowercase().contains(&query_lower) {
                return true;
            }
        }

        false
    }
}

/// 市场统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketStats {
    /// 规则总数
    pub total_rules: usize,
    /// 总下载次数
    pub total_downloads: u64,
    /// 分类列表
    pub categories: Vec<String>,
}

/// 市场错误类型
#[derive(Debug, Clone, thiserror::Error)]
pub enum MarketError {
    /// 规则未找到
    #[error("规则未找到: {0}")]
    NotFound(String),

    /// 上传失败
    #[error("上传失败: {0}")]
    UploadFailed(String),

    /// 下载失败
    #[error("下载失败: {0}")]
    DownloadFailed(String),

    /// 网络错误
    #[error("网络错误: {0}")]
    NetworkError(String),

    /// 解析错误
    #[error("解析错误: {0}")]
    ParseError(String),

    /// 无效输入
    #[error("无效输入: {0}")]
    InvalidInput(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marketplace_creation() {
        let config = MarketConfig::default();
        let market = Marketplace::new(config);
        assert_eq!(market.config.name, "World Rules Marketplace");
    }

    #[test]
    fn test_rule_package_creation() {
        let package = RulePackage::new(
            "mahjong-sichuan".to_string(),
            "四川麻将".to_string(),
            "games".to_string(),
        );

        assert_eq!(package.id, "mahjong-sichuan");
        assert_eq!(package.name, "四川麻将");
        assert_eq!(package.category, "games");
        assert_eq!(package.version, "1.0.0");
    }

    #[test]
    fn test_rule_package_matching() {
        let mut package = RulePackage::new(
            "mahjong-sichuan".to_string(),
            "四川麻将".to_string(),
            "games".to_string(),
        );
        package.tags.push("麻将".to_string());

        assert!(package.matches("麻将"));
        assert!(package.matches("四川"));
        assert!(!package.matches("扑克"));
    }

    #[test]
    fn test_marketplace_upload_and_search() {
        let mut market = Marketplace::new(MarketConfig::default());

        let mut package = RulePackage::new(
            "mahjong-sichuan".to_string(),
            "四川麻将".to_string(),
            "games".to_string(),
        );
        package.tags.push("麻将".to_string());

        // 上传规则
        let id = market.upload(package).unwrap();
        assert_eq!(id, "mahjong-sichuan");

        // 搜索规则
        let results = market.search("麻将").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "四川麻将");
    }

    #[test]
    fn test_marketplace_stats() {
        let mut market = Marketplace::new(MarketConfig::default());

        let package = RulePackage::new(
            "mahjong-sichuan".to_string(),
            "四川麻将".to_string(),
            "games".to_string(),
        );

        market.upload(package).unwrap();

        let stats = market.stats();
        assert_eq!(stats.total_rules, 1);
    }

    #[test]
    fn test_marketplace_download() {
        let mut market = Marketplace::new(MarketConfig::default());

        let package = RulePackage::new(
            "mahjong-sichuan".to_string(),
            "四川麻将".to_string(),
            "games".to_string(),
        );

        market.upload(package).unwrap();

        let downloaded = market.download("mahjong-sichuan");
        assert!(downloaded.is_some());
        assert_eq!(downloaded.unwrap().name, "四川麻将");
    }
}
