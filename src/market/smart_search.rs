//! # 高级搜索功能
//!
//! 提供智能搜索、模糊匹配、搜索建议等高级功能。

use super::search::SearchResult;
use super::types::RulePackage;
use std::collections::HashMap;

/// 智能搜索引擎
#[derive(Debug)]
pub struct SmartSearchEngine {
    /// 搜索历史（用户ID -> 搜索关键词列表）
    search_history: HashMap<String, Vec<String>>,
    /// 热门搜索词
    hot_keywords: HashMap<String, u64>,
    /// 同义词词典
    synonyms: HashMap<String, Vec<String>>,
}

impl SmartSearchEngine {
    /// 创建新的智能搜索引擎
    pub fn new() -> Self {
        let mut synonyms = HashMap::new();

        // 添加同义词（中文）
        synonyms.insert(
            "麻将".to_string(),
            vec!["麻雀".to_string(), "mahjong".to_string()],
        );
        synonyms.insert(
            "扑克".to_string(),
            vec!["poker".to_string(), "纸牌".to_string()],
        );
        synonyms.insert(
            "象棋".to_string(),
            vec!["中国象棋".to_string(), "chinese chess".to_string()],
        );
        synonyms.insert(
            "围棋".to_string(),
            vec!["go".to_string(), "weiqi".to_string()],
        );

        Self {
            search_history: HashMap::new(),
            hot_keywords: HashMap::new(),
            synonyms,
        }
    }

    /// 执行智能搜索
    ///
    /// # Arguments
    ///
    /// * `query` - 搜索关键词
    /// * `packages` - 所有规则包
    /// * `user_id` - 用户ID（可选）
    pub fn search(
        &mut self,
        query: &str,
        packages: &[RulePackage],
        user_id: Option<&str>,
    ) -> SearchResult {
        let start = std::time::Instant::now();

        // 记录搜索历史
        if let Some(uid) = user_id {
            self.record_search_history(uid, query);
        }

        // 更新热门搜索词
        self.hot_keywords
            .entry(query.to_lowercase())
            .or_insert(0)
            .add_assign(1);

        // 扩展查询（添加同义词）
        let expanded_queries = self.expand_query(query);

        // 执行搜索
        let mut results: Vec<RulePackage> = packages
            .iter()
            .filter(|pkg| expanded_queries.iter().any(|q| pkg.matches(q)))
            .cloned()
            .collect();

        // 按相关度排序
        results.sort_by(|a, b| {
            let score_a = self.calculate_relevance_score(query, a);
            let score_b = self.calculate_relevance_score(query, b);
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let total = results.len();

        SearchResult {
            packages: results,
            total,
            took_ms: start.elapsed().as_millis() as u64,
        }
    }

    /// 获取搜索建议
    ///
    /// # Arguments
    ///
    /// * `query` - 部分搜索关键词
    /// * `limit` - 返回数量限制
    pub fn get_suggestions(&self, query: &str, limit: usize) -> Vec<String> {
        let query_lower = query.to_lowercase();

        let mut suggestions: Vec<String> = self
            .hot_keywords
            .keys()
            .filter(|keyword| keyword.starts_with(&query_lower))
            .take(limit)
            .cloned()
            .collect();

        // 添加同义词建议
        for (key, syns) in &self.synonyms {
            if key.starts_with(&query_lower) {
                suggestions.push(key.clone());
                suggestions.extend(syns.iter().filter(|s| s.starts_with(&query_lower)).cloned());
            }
        }

        suggestions.sort();
        suggestions.dedup();
        suggestions.truncate(limit);

        suggestions
    }

    /// 获取热门搜索词
    ///
    /// # Arguments
    ///
    /// * `limit` - 返回数量限制
    pub fn get_hot_searches(&self, limit: usize) -> Vec<(String, u64)> {
        let mut hot: Vec<(String, u64)> = self
            .hot_keywords
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();

        hot.sort_by_key(|b| std::cmp::Reverse(b.1));
        hot.truncate(limit);

        hot
    }

    /// 获取用户搜索历史
    ///
    /// # Arguments
    ///
    /// * `user_id` - 用户ID
    /// * `limit` - 返回数量限制
    pub fn get_search_history(&self, user_id: &str, limit: usize) -> Vec<String> {
        self.search_history
            .get(user_id)
            .map(|history| history.iter().rev().take(limit).cloned().collect())
            .unwrap_or_default()
    }

    /// 清除用户搜索历史
    ///
    /// # Arguments
    ///
    /// * `user_id` - 用户ID
    pub fn clear_search_history(&mut self, user_id: &str) {
        self.search_history.remove(user_id);
    }

    /// 添加同义词
    ///
    /// # Arguments
    ///
    /// * `word` - 原词
    /// * `synonyms` - 同义词列表
    pub fn add_synonyms(&mut self, word: impl Into<String>, synonyms: Vec<String>) {
        self.synonyms.insert(word.into(), synonyms);
    }

    /// 扩展查询（添加同义词）
    fn expand_query(&self, query: &str) -> Vec<String> {
        let mut queries = vec![query.to_string()];

        // 查找同义词
        let query_lower = query.to_lowercase();
        for (key, syns) in &self.synonyms {
            if key == &query_lower {
                queries.extend(syns.clone());
            } else if syns.iter().any(|s| s.to_lowercase() == query_lower) {
                queries.push(key.clone());
                queries.extend(syns.iter().filter(|s| s != &query).cloned());
            }
        }

        queries
    }

    /// 计算相关度得分
    fn calculate_relevance_score(&self, query: &str, package: &RulePackage) -> f32 {
        let query_lower = query.to_lowercase();

        let mut score = 0.0;

        // 名称完全匹配
        if package.name.to_lowercase() == query_lower {
            score += 10.0;
        }
        // 名称包含
        else if package.name.to_lowercase().contains(&query_lower) {
            score += 5.0;
        }

        // 标签匹配
        for tag in &package.tags {
            if tag.to_lowercase() == query_lower {
                score += 3.0;
            } else if tag.to_lowercase().contains(&query_lower) {
                score += 1.0;
            }
        }

        // 描述匹配
        if package.description.to_lowercase().contains(&query_lower) {
            score += 1.0;
        }

        // 评分加成
        score += package.rating * 0.5;

        // 下载量加成
        if package.downloads > 0 {
            score += (package.downloads as f32).log10() * 0.1;
        }

        score
    }

    /// 记录搜索历史
    fn record_search_history(&mut self, user_id: &str, query: &str) {
        self.search_history
            .entry(user_id.to_string())
            .or_default()
            .push(query.to_string());

        // 限制历史记录数量（最近100条）
        if let Some(history) = self.search_history.get_mut(user_id) {
            if history.len() > 100 {
                history.remove(0);
            }
        }
    }
}

impl Default for SmartSearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 搜索分析器
#[derive(Debug)]
pub struct SearchAnalyzer {
    /// 搜索统计
    stats: HashMap<String, SmartSearchStats>,
}

impl SearchAnalyzer {
    /// 创建新的搜索分析器
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    /// 记录搜索
    ///
    /// # Arguments
    ///
    /// * `query` - 搜索关键词
    /// * `results_count` - 结果数量
    pub fn record_search(&mut self, query: &str, results_count: usize) {
        let stat = self
            .stats
            .entry(query.to_lowercase())
            .or_insert_with(|| SmartSearchStats {
                query: query.to_string(),
                total_searches: 0,
                avg_results: 0.0,
                no_results_count: 0,
            });

        stat.total_searches += 1;
        stat.avg_results = (stat.avg_results * (stat.total_searches - 1) as f32
            + results_count as f32)
            / stat.total_searches as f32;

        if results_count == 0 {
            stat.no_results_count += 1;
        }
    }

    /// 获取搜索统计
    ///
    /// # Arguments
    ///
    /// * `query` - 搜索关键词
    pub fn get_stats(&self, query: &str) -> Option<&SmartSearchStats> {
        self.stats.get(&query.to_lowercase())
    }

    /// 获取无结果搜索
    pub fn get_zero_result_searches(&self) -> Vec<&SmartSearchStats> {
        self.stats
            .values()
            .filter(|s| s.avg_results < 1.0)
            .collect()
    }

    /// 获取热门搜索
    ///
    /// # Arguments
    ///
    /// * `limit` - 返回数量限制
    pub fn get_top_searches(&self, limit: usize) -> Vec<&SmartSearchStats> {
        let mut top: Vec<&SmartSearchStats> = self.stats.values().collect();
        top.sort_by_key(|b| std::cmp::Reverse(b.total_searches));
        top.truncate(limit);
        top
    }
}

impl Default for SearchAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// 智能搜索统计
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmartSearchStats {
    /// 搜索关键词
    pub query: String,
    /// 总搜索次数
    pub total_searches: u64,
    /// 平均结果数
    pub avg_results: f32,
    /// 无结果次数
    pub no_results_count: u64,
}

// 辅助trait
trait AddAssign<Rhs> {
    fn add_assign(&mut self, rhs: Rhs);
}

impl AddAssign<u64> for u64 {
    fn add_assign(&mut self, rhs: u64) {
        *self += rhs;
    }
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
    fn test_smart_search_engine_creation() {
        let engine = SmartSearchEngine::new();
        assert!(!engine.synonyms.is_empty());
    }

    #[test]
    fn test_smart_search_engine_search() {
        let mut engine = SmartSearchEngine::new();

        let package = create_test_package("test-1", "四川麻将", "games");
        let packages = vec![package];

        let result = engine.search("麻将", &packages, Some("user-1"));
        assert_eq!(result.packages.len(), 1);
    }

    #[test]
    fn test_smart_search_engine_synonyms() {
        let mut engine = SmartSearchEngine::new();

        let package = create_test_package("test-1", "四川麻将", "games");
        let packages = vec![package];

        // 搜索同义词
        let result = engine.search("mahjong", &packages, None);
        assert_eq!(result.packages.len(), 1);
    }

    #[test]
    fn test_smart_search_engine_suggestions() {
        let mut engine = SmartSearchEngine::new();

        // 执行一些搜索以建立热门词
        engine.search("麻将", &[], None);
        engine.search("麻将", &[], None);
        engine.search("扑克", &[], None);

        let suggestions = engine.get_suggestions("麻", 10);
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("麻将")));
    }

    #[test]
    fn test_smart_search_engine_hot_searches() {
        let mut engine = SmartSearchEngine::new();

        // 执行多次搜索
        engine.search("麻将", &[], None);
        engine.search("麻将", &[], None);
        engine.search("扑克", &[], None);

        let hot = engine.get_hot_searches(10);
        assert!(!hot.is_empty());
        // "麻将"应该是最热门的
        assert_eq!(hot[0].0, "麻将");
    }

    #[test]
    fn test_smart_search_engine_history() {
        let mut engine = SmartSearchEngine::new();

        engine.search("麻将", &[], Some("user-1"));
        engine.search("扑克", &[], Some("user-1"));

        let history = engine.get_search_history("user-1", 10);
        assert_eq!(history.len(), 2);
        assert_eq!(history[0], "扑克"); // 最新的一条
    }

    #[test]
    fn test_smart_search_engine_clear_history() {
        let mut engine = SmartSearchEngine::new();

        engine.search("麻将", &[], Some("user-1"));
        engine.clear_search_history("user-1");

        let history = engine.get_search_history("user-1", 10);
        assert!(history.is_empty());
    }

    #[test]
    fn test_smart_search_engine_add_synonyms() {
        let mut engine = SmartSearchEngine::new();
        engine.add_synonyms("测试", vec!["test".to_string(), "testing".to_string()]);

        assert!(engine.synonyms.contains_key("测试"));
    }

    #[test]
    fn test_smart_search_engine_relevance() {
        let mut engine = SmartSearchEngine::new();

        let mut package1 = create_test_package("test-1", "四川麻将", "games");
        package1.rating = 5.0;
        package1.downloads = 1000;

        let mut package2 = create_test_package("test-2", "麻将入门", "games");
        package2.rating = 3.0;
        package2.downloads = 100;

        let packages = vec![package1, package2];
        let result = engine.search("麻将", &packages, None);

        // 高评分的应该排在前面
        assert_eq!(result.packages[0].rating, 5.0);
    }

    #[test]
    fn test_smart_search_engine_default() {
        let engine = SmartSearchEngine::default();
        assert!(!engine.synonyms.is_empty());
    }

    #[test]
    fn test_search_analyzer_creation() {
        let analyzer = SearchAnalyzer::new();
        assert!(analyzer.stats.is_empty());
    }

    #[test]
    fn test_search_analyzer_record_search() {
        let mut analyzer = SearchAnalyzer::new();

        analyzer.record_search("麻将", 10);
        analyzer.record_search("麻将", 5);

        let stats = analyzer.get_stats("麻将");
        assert!(stats.is_some());

        let stats = stats.unwrap();
        assert_eq!(stats.total_searches, 2);
        assert!((stats.avg_results - 7.5).abs() < 0.1);
    }

    #[test]
    fn test_search_analyzer_zero_results() {
        let mut analyzer = SearchAnalyzer::new();

        analyzer.record_search("不存在的规则", 0);
        analyzer.record_search("不存在的规则", 0);

        let zero_results = analyzer.get_zero_result_searches();
        assert_eq!(zero_results.len(), 1);
    }

    #[test]
    fn test_search_analyzer_top_searches() {
        let mut analyzer = SearchAnalyzer::new();

        analyzer.record_search("麻将", 10);
        analyzer.record_search("麻将", 10);
        analyzer.record_search("麻将", 10);
        analyzer.record_search("扑克", 10);

        let top = analyzer.get_top_searches(10);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].query, "麻将");
        assert_eq!(top[0].total_searches, 3);
    }

    #[test]
    fn test_search_analyzer_default() {
        let analyzer = SearchAnalyzer::default();
        assert!(analyzer.stats.is_empty());
    }
}
