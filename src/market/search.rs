//! # 规则市场搜索功能
//!
//! 提供高级搜索和过滤功能。

use super::types::{MarketError, RulePackage};
use std::collections::HashMap;

/// 搜索过滤器
#[derive(Debug, Clone, Default)]
pub struct SearchFilter {
    /// 分类过滤
    pub category: Option<String>,
    /// 作者过滤
    pub author: Option<String>,
    /// 最低评分
    pub min_rating: Option<f32>,
    /// 标签过滤
    pub tags: Vec<String>,
    /// 关键词
    pub keywords: Vec<String>,
}

impl SearchFilter {
    /// 创建新的搜索过滤器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置分类过滤
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// 设置作者过滤
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// 设置最低评分
    pub fn min_rating(mut self, rating: f32) -> Self {
        self.min_rating = Some(rating);
        self
    }

    /// 添加标签
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// 添加关键词
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keywords.push(keyword.into());
        self
    }

    /// 检查规则是否匹配过滤器
    pub fn matches(&self, package: &RulePackage) -> bool {
        // 检查分类
        if let Some(ref cat) = self.category {
            if package.category != *cat {
                return false;
            }
        }

        // 检查作者
        if let Some(ref author) = self.author {
            if package.author != *author {
                return false;
            }
        }

        // 检查评分
        if let Some(min_rating) = self.min_rating {
            if package.rating < min_rating {
                return false;
            }
        }

        // 检查标签
        for tag in &self.tags {
            if !package.tags.contains(tag) {
                return false;
            }
        }

        // 检查关键词
        for keyword in &self.keywords {
            if !package.matches(keyword) {
                return false;
            }
        }

        true
    }
}

/// 搜索结果
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// 匹配的规则包
    pub packages: Vec<RulePackage>,
    /// 总数
    pub total: usize,
    /// 搜索耗时（毫秒）
    pub took_ms: u64,
}

/// 搜索引擎
#[derive(Debug)]
pub struct SearchEngine {
    /// 倒排索引（词 -> 规则ID列表）
    inverted_index: HashMap<String, Vec<String>>,
    /// 规则缓存
    rules: HashMap<String, RulePackage>,
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchEngine {
    /// 创建新的搜索引擎
    pub fn new() -> Self {
        Self {
            inverted_index: HashMap::new(),
            rules: HashMap::new(),
        }
    }

    /// 添加规则到索引
    pub fn index(&mut self, package: RulePackage) {
        let id = package.id.clone();

        // 建立倒排索引
        self.index_field(&id, &package.name);
        self.index_field(&id, &package.description);

        for tag in &package.tags {
            self.index_field(&id, tag);
        }

        // 存储规则
        self.rules.insert(id, package);
    }

    fn index_field(&mut self, rule_id: &str, text: &str) {
        // 简单分词（按空格和标点）
        for word in text.split_whitespace() {
            let word_lower = word.to_lowercase();
            self.inverted_index
                .entry(word_lower)
                .or_default()
                .push(rule_id.to_string());
        }

        // 对于中文字符串，进行字符级索引（支持部分匹配）
        let chars: Vec<char> = text.chars().collect();
        // 索引 2-4 字符的子串
        for len in 2..=4.min(chars.len()) {
            for i in 0..=chars.len() - len {
                let substr: String = chars[i..i + len].iter().collect();
                self.inverted_index
                    .entry(substr)
                    .or_default()
                    .push(rule_id.to_string());
            }
        }
    }

    /// 执行搜索
    pub fn search(&self, filter: &SearchFilter) -> Result<SearchResult, MarketError> {
        let start = std::time::Instant::now();

        let mut results: Vec<RulePackage> = if filter.keywords.is_empty() {
            // 如果没有关键词，返回所有规则
            self.rules.values().cloned().collect()
        } else {
            // 使用倒排索引搜索
            let mut candidate_ids: Option<Vec<String>> = None;

            for keyword in &filter.keywords {
                let keyword_lower = keyword.to_lowercase();

                if let Some(ids) = self.inverted_index.get(&keyword_lower) {
                    match candidate_ids {
                        Some(ref mut existing) => {
                            // 取交集
                            let ids_set: std::collections::HashSet<_> =
                                ids.iter().cloned().collect();
                            existing.retain(|id| ids_set.contains(id));
                        }
                        None => {
                            candidate_ids = Some(ids.clone());
                        }
                    }
                } else {
                    // 没有匹配的关键词，返回空结果
                    return Ok(SearchResult {
                        packages: Vec::new(),
                        total: 0,
                        took_ms: start.elapsed().as_millis() as u64,
                    });
                }
            }

            // 获取规则包
            if let Some(ids) = candidate_ids {
                ids.into_iter()
                    .filter_map(|id| self.rules.get(&id).cloned())
                    .collect()
            } else {
                Vec::new()
            }
        };

        // 应用过滤器
        results.retain(|pkg| filter.matches(pkg));

        // 按评分排序
        results.sort_by(|a, b| {
            b.rating
                .partial_cmp(&a.rating)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let total = results.len();

        Ok(SearchResult {
            packages: results,
            total,
            took_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// 获取统计信息
    pub fn stats(&self) -> SearchStats {
        SearchStats {
            total_indexed: self.rules.len(),
            vocabulary_size: self.inverted_index.len(),
        }
    }
}

/// 搜索统计
#[derive(Debug, Clone)]
pub struct SearchStats {
    /// 已索引规则数
    pub total_indexed: usize,
    /// 词汇表大小
    pub vocabulary_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_package(id: &str, name: &str, category: &str) -> RulePackage {
        let mut package = RulePackage::new(id.to_string(), name.to_string(), category.to_string());
        package.tags.push(category.to_string());
        package
    }

    #[test]
    fn test_search_filter_basic() {
        let package = create_test_package("test-1", "四川麻将", "games");

        let filter = SearchFilter::new().category("games");
        assert!(filter.matches(&package));

        let filter = SearchFilter::new().category("sports");
        assert!(!filter.matches(&package));
    }

    #[test]
    fn test_search_filter_rating() {
        let mut package = create_test_package("test-1", "四川麻将", "games");
        package.rating = 4.5;

        let filter = SearchFilter::new().min_rating(4.0);
        assert!(filter.matches(&package));

        let filter = SearchFilter::new().min_rating(5.0);
        assert!(!filter.matches(&package));
    }

    #[test]
    fn test_search_filter_tags() {
        let package = create_test_package("test-1", "四川麻将", "games");

        let filter = SearchFilter::new().tag("games");
        assert!(filter.matches(&package));

        let filter = SearchFilter::new().tag("sports").tag("games");
        assert!(!filter.matches(&package));
    }

    #[test]
    fn test_search_engine_indexing() {
        let mut engine = SearchEngine::new();

        let package = create_test_package("test-1", "四川麻将", "games");
        engine.index(package);

        let stats = engine.stats();
        assert_eq!(stats.total_indexed, 1);
    }

    #[test]
    fn test_search_engine_search() {
        let mut engine = SearchEngine::new();

        let package1 = create_test_package("test-1", "四川麻将", "games");
        let package2 = create_test_package("test-2", "篮球规则", "sports");

        engine.index(package1);
        engine.index(package2);

        let filter = SearchFilter::new().keyword("麻将");
        let result = engine.search(&filter).unwrap();

        assert_eq!(result.packages.len(), 1);
        assert_eq!(result.packages[0].name, "四川麻将");
    }

    #[test]
    fn test_search_engine_filter() {
        let mut engine = SearchEngine::new();

        let mut package1 = create_test_package("test-1", "四川麻将", "games");
        package1.rating = 4.5;

        let mut package2 = create_test_package("test-2", "国标麻将", "games");
        package2.rating = 3.0;

        engine.index(package1);
        engine.index(package2);

        let filter = SearchFilter::new().category("games").min_rating(4.0);
        let result = engine.search(&filter).unwrap();

        assert_eq!(result.packages.len(), 1);
        assert_eq!(result.packages[0].name, "四川麻将");
    }

    #[test]
    fn test_search_result_timing() {
        let engine = SearchEngine::new();

        let filter = SearchFilter::new();
        let result = engine.search(&filter).unwrap();

        // 搜索应该非常快（小于100ms）
        assert!(result.took_ms < 100);
    }
}
