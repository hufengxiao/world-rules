//! 轮椅橄榄球规则
//!
//! 轮椅橄榄球是一项混合性别团队运动。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 轮椅橄榄球规则
pub struct WheelchairRugbyRules {
    metadata: RuleMetadata,
}

impl WheelchairRugbyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("轮椅橄榄球规则", "轮椅橄榄球比赛规则")
                .with_origin("IWRF/IPC")
                .with_tags(vec!["体育".into(), "轮椅".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "0.5-3.5分分级系统",
            "0.5分: 最严重四肢瘫",
            "3.5分: 轻度功能障碍",
            "总分限制: 8.0分（场上4人）",
            "分级评估: 功能测试",
            "女性运动员: +0.5分优惠",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "残奥会: 单项",
            "世界锦标赛",
            "洲际锦标赛",
            "国内联赛",
            "比赛制: 4节×8分钟",
            "混合性别: 男女同场",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "轮椅: 攻击型/防守型",
            "攻击轮椅: 前端凸起",
            "防守轮椅: 后端凸起",
            "球: 标准排球",
            "手套: 允许",
            "固定装置: 允许",
            "禁止: 电子辅助",
        ]
    }

    /// 比赛规则
    pub fn gameplay(&self) -> Vec<&'static str> {
        vec![
            "场地: 室内篮球场",
            "目标区: 两端各一个",
            "得分: 持球进入目标区",
            "传球: 必须传球（10秒内）",
            "运球: 可推球或运球",
            "犯规限制: 4次犯规出局",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "非法碰撞",
            "背后冲撞",
            "持球超时（10秒）",
            "未传球",
            "越出场地",
            "目标区违规停留",
        ]
    }

    /// 安全规则
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "禁止危险碰撞",
            "轮椅安全标准",
            "固定装置检查",
            "医疗支持在场",
            "犯规累积: 4次退场",
            "比赛暂停: 紧急情况",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "四肢瘫或类似残疾",
            "IWRF分级认证",
            "最低功能要求",
            "国际分级证书",
            "国家队注册",
        ]
    }
}

impl Default for WheelchairRugbyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WheelchairRugbyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wheelchair_rugby")
    }

    fn explain(&self) -> String {
        format!(
            "【轮椅橄榄球规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            装备要求:\n{}\n\n\
            比赛规则:\n{}",
            self.classification()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.equipment()
                .iter()
                .map(|eq| format!("  • {}", eq))
                .collect::<Vec<_>>()
                .join("\n"),
            self.gameplay()
                .iter()
                .map(|g| format!("  • {}", g))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wheelchair_rugby_rules_basic() {
        let rules = WheelchairRugbyRules::new();
        assert_eq!(rules.metadata().name, "轮椅橄榄球规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_wheelchair_rugby_classification() {
        let rules = WheelchairRugbyRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("分")));
        assert!(classification.iter().any(|c| c.contains("0.5")));
        assert!(classification.iter().any(|c| c.contains("3.5")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_wheelchair_rugby_events() {
        let rules = WheelchairRugbyRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("残奥会")));
        assert!(events.iter().any(|e| e.contains("世界锦标赛")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_wheelchair_rugby_gameplay() {
        let rules = WheelchairRugbyRules::new();
        let gameplay = rules.gameplay();
        assert!(gameplay.iter().any(|g| g.contains("场地")));
        assert!(gameplay.iter().any(|g| g.contains("得分")));
        assert!(gameplay.len() >= 4);
    }

    #[test]
    fn test_wheelchair_rugby_category() {
        let rules = WheelchairRugbyRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
