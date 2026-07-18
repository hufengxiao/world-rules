//! # 规则评分系统
//!
//! 提供规则评分、评论、推荐功能。

use super::types::{MarketError, RulePackage};
use std::collections::HashMap;

/// 评分管理器
#[derive(Debug)]
pub struct RatingManager {
    /// 规则评分（规则ID -> 评分列表）
    ratings: HashMap<String, Vec<Rating>>,
    /// 平均评分缓存
    average_ratings: HashMap<String, f32>,
}

impl RatingManager {
    /// 创建新的评分管理器
    pub fn new() -> Self {
        Self {
            ratings: HashMap::new(),
            average_ratings: HashMap::new(),
        }
    }

    /// 添加评分
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 规则ID
    /// * `user_id` - 用户ID
    /// * `score` - 评分（1-5）
    /// * `comment` - 评论
    pub fn add_rating(
        &mut self,
        rule_id: impl Into<String>,
        user_id: impl Into<String>,
        score: u8,
        comment: impl Into<String>,
    ) -> Result<(), MarketError> {
        // 验证评分范围
        if !(1..=5).contains(&score) {
            return Err(MarketError::InvalidInput(format!(
                "评分必须在 1-5 之间，当前: {}",
                score
            )));
        }

        let rule_id = rule_id.into();
        let user_id = user_id.into();
        let comment = comment.into();

        let rating = Rating {
            user_id,
            score,
            comment,
            created_at: chrono::Utc::now(),
        };

        // 添加评分
        self.ratings
            .entry(rule_id.clone())
            .or_default()
            .push(rating);

        // 更新平均评分缓存
        self.update_average_rating(&rule_id);

        Ok(())
    }

    /// 获取规则的平均评分
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 规则ID
    pub fn get_average_rating(&self, rule_id: &str) -> Option<f32> {
        self.average_ratings.get(rule_id).copied()
    }

    /// 获取规则的所有评分
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 规则ID
    pub fn get_ratings(&self, rule_id: &str) -> Option<&[Rating]> {
        self.ratings.get(rule_id).map(|v| v.as_slice())
    }

    /// 获取规则的评分数量
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 规则ID
    pub fn rating_count(&self, rule_id: &str) -> usize {
        self.ratings.get(rule_id).map(|v| v.len()).unwrap_or(0)
    }

    /// 更新平均评分缓存
    fn update_average_rating(&mut self, rule_id: &str) {
        if let Some(ratings) = self.ratings.get(rule_id) {
            if ratings.is_empty() {
                self.average_ratings.insert(rule_id.to_string(), 0.0);
            } else {
                let total: u32 = ratings.iter().map(|r| r.score as u32).sum();
                let average = total as f32 / ratings.len() as f32;
                self.average_ratings
                    .insert(rule_id.to_string(), (average * 10.0).round() / 10.0);
                // 保留一位小数
            }
        }
    }

    /// 获取评分统计
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 规则ID
    pub fn get_rating_stats(&self, rule_id: &str) -> Option<RatingStats> {
        let ratings = self.ratings.get(rule_id)?;

        let mut score_distribution = [0usize; 5];
        for rating in ratings {
            if rating.score >= 1 && rating.score <= 5 {
                score_distribution[(rating.score - 1) as usize] += 1;
            }
        }

        Some(RatingStats {
            total_ratings: ratings.len(),
            average_rating: self.get_average_rating(rule_id).unwrap_or(0.0),
            score_distribution,
        })
    }
}

impl Default for RatingManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 评分信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Rating {
    /// 用户ID
    pub user_id: String,
    /// 评分（1-5）
    pub score: u8,
    /// 评论内容
    pub comment: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 评分统计
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RatingStats {
    /// 总评分数
    pub total_ratings: usize,
    /// 平均评分
    pub average_rating: f32,
    /// 评分分布（索引0-4对应评分1-5）
    pub score_distribution: [usize; 5],
}

/// 推荐引擎
#[derive(Debug)]
pub struct RecommendationEngine {
    /// 用户偏好（用户ID -> 分类偏好）
    user_preferences: HashMap<String, HashMap<String, f32>>,
}

impl RecommendationEngine {
    /// 创建新的推荐引擎
    pub fn new() -> Self {
        Self {
            user_preferences: HashMap::new(),
        }
    }

    /// 更新用户偏好
    ///
    /// # Arguments
    ///
    /// * `user_id` - 用户ID
    /// * `category` - 分类
    /// * `weight` - 权重（通常基于用户行为）
    pub fn update_preference(
        &mut self,
        user_id: impl Into<String>,
        category: impl Into<String>,
        weight: f32,
    ) {
        let user_id = user_id.into();
        let category = category.into();

        self.user_preferences
            .entry(user_id)
            .or_default()
            .insert(category, weight);
    }

    /// 获取推荐规则
    ///
    /// # Arguments
    ///
    /// * `user_id` - 用户ID
    /// * `packages` - 所有规则包
    /// * `limit` - 返回数量限制
    pub fn get_recommendations(
        &self,
        user_id: &str,
        packages: &[RulePackage],
        limit: usize,
    ) -> Vec<RulePackage> {
        let preferences = self.user_preferences.get(user_id);

        // 计算每个规则的得分
        let mut scored_packages: Vec<(f32, RulePackage)> = packages
            .iter()
            .map(|pkg| {
                let score = self.calculate_score(pkg, preferences);
                (score, pkg.clone())
            })
            .collect();

        // 按得分降序排序
        scored_packages.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        // 返回前 N 个
        scored_packages
            .into_iter()
            .take(limit)
            .map(|(_, pkg)| pkg)
            .collect()
    }

    /// 计算规则得分
    fn calculate_score(
        &self,
        package: &RulePackage,
        preferences: Option<&HashMap<String, f32>>,
    ) -> f32 {
        // 基础得分：评分
        let base_score = package.rating;

        // 偏好加成
        let preference_bonus = preferences
            .and_then(|prefs| prefs.get(&package.category))
            .unwrap_or(&0.0);

        // 下载量加成（归一化）
        let download_bonus = if package.downloads > 0 {
            (package.downloads as f32).log10() / 10.0
        } else {
            0.0
        };

        base_score + preference_bonus + download_bonus
    }
}

impl Default for RecommendationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rating_manager_creation() {
        let manager = RatingManager::new();
        assert_eq!(manager.rating_count("test"), 0);
    }

    #[test]
    fn test_rating_manager_add_rating() {
        let mut manager = RatingManager::new();

        let result = manager.add_rating("test-1", "user-1", 5, "优秀");
        assert!(result.is_ok());

        assert_eq!(manager.rating_count("test-1"), 1);
    }

    #[test]
    fn test_rating_manager_invalid_score() {
        let mut manager = RatingManager::new();

        let result = manager.add_rating("test-1", "user-1", 0, "无效评分");
        assert!(result.is_err());

        let result = manager.add_rating("test-1", "user-1", 6, "无效评分");
        assert!(result.is_err());
    }

    #[test]
    fn test_rating_manager_average_rating() {
        let mut manager = RatingManager::new();

        manager.add_rating("test-1", "user-1", 5, "").unwrap();
        manager.add_rating("test-1", "user-2", 3, "").unwrap();
        manager.add_rating("test-1", "user-3", 4, "").unwrap();

        let avg = manager.get_average_rating("test-1").unwrap();
        assert!((avg - 4.0).abs() < 0.1);
    }

    #[test]
    fn test_rating_manager_get_ratings() {
        let mut manager = RatingManager::new();

        manager.add_rating("test-1", "user-1", 5, "优秀").unwrap();

        let ratings = manager.get_ratings("test-1");
        assert!(ratings.is_some());
        assert_eq!(ratings.unwrap().len(), 1);
    }

    #[test]
    fn test_rating_manager_rating_stats() {
        let mut manager = RatingManager::new();

        manager.add_rating("test-1", "user-1", 5, "").unwrap();
        manager.add_rating("test-1", "user-2", 5, "").unwrap();
        manager.add_rating("test-1", "user-3", 3, "").unwrap();

        let stats = manager.get_rating_stats("test-1");
        assert!(stats.is_some());

        let stats = stats.unwrap();
        assert_eq!(stats.total_ratings, 3);
        assert_eq!(stats.score_distribution[4], 2); // 5星有2个
        assert_eq!(stats.score_distribution[2], 1); // 3星有1个
    }

    #[test]
    fn test_rating_manager_default() {
        let manager = RatingManager::default();
        assert_eq!(manager.rating_count("test"), 0);
    }

    #[test]
    fn test_recommendation_engine_creation() {
        let engine = RecommendationEngine::new();
        assert!(engine.user_preferences.is_empty());
    }

    #[test]
    fn test_recommendation_engine_update_preference() {
        let mut engine = RecommendationEngine::new();

        engine.update_preference("user-1", "games", 2.0);

        assert!(engine.user_preferences.contains_key("user-1"));
        assert_eq!(engine.user_preferences["user-1"]["games"], 2.0);
    }

    #[test]
    fn test_recommendation_engine_get_recommendations() {
        let mut engine = RecommendationEngine::new();
        engine.update_preference("user-1", "games", 2.0);

        let mut package1 = RulePackage::new(
            "test-1".to_string(),
            "游戏规则".to_string(),
            "games".to_string(),
        );
        package1.rating = 4.5;
        package1.downloads = 100;

        let mut package2 = RulePackage::new(
            "test-2".to_string(),
            "体育规则".to_string(),
            "sports".to_string(),
        );
        package2.rating = 4.0;
        package2.downloads = 50;

        let packages = vec![package1, package2];
        let recommendations = engine.get_recommendations("user-1", &packages, 2);

        assert_eq!(recommendations.len(), 2);
        // 游戏规则应该排在前面（有偏好加成）
        assert_eq!(recommendations[0].category, "games");
    }

    #[test]
    fn test_recommendation_engine_no_preferences() {
        let engine = RecommendationEngine::new();

        let mut package1 = RulePackage::new(
            "test-1".to_string(),
            "高评分".to_string(),
            "games".to_string(),
        );
        package1.rating = 5.0;

        let mut package2 = RulePackage::new(
            "test-2".to_string(),
            "低评分".to_string(),
            "games".to_string(),
        );
        package2.rating = 3.0;

        let packages = vec![package1.clone(), package2.clone()];
        let recommendations = engine.get_recommendations("user-1", &packages, 2);

        assert_eq!(recommendations.len(), 2);
        // 按评分排序
        assert_eq!(recommendations[0].rating, 5.0);
    }

    #[test]
    fn test_recommendation_engine_default() {
        let engine = RecommendationEngine::default();
        assert!(engine.user_preferences.is_empty());
    }
}
