//! 残疾人跆拳道规则
//!
//! 残疾人跆拳道是残奥会新项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人跆拳道规则
pub struct ParaTaekwondoRules {
    metadata: RuleMetadata,
}

impl ParaTaekwondoRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人跆拳道规则", "残疾人跆拳道比赛规则")
                .with_origin("WP/WT")
                .with_tags(vec!["体育".into(), "跆拳道".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "K41: 上肢缺失或功能障碍（最严重）",
            "K42: 中度上肢功能障碍",
            "K43: 轻度上肢功能障碍",
            "K44: 最低残疾标准",
            "分级评估: 功能测试",
            "性别分组: 男女分开",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "残奥会: K41-K44级别",
            "世界锦标赛",
            "洲际锦标赛",
            "世界杯赛",
            "比赛制: 单败淘汰",
            "体重分级: 男女各4级",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "道服: WT认证",
            "护具: 头盔、护甲",
            "护腿: 强制",
            "护臂: 强制",
            "护齿: 强制",
            "手套: WT认证",
            "感应袜: 电子计分",
        ]
    }

    /// 技术规则
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "踢击: 允许（主要得分）",
            "拳打: 躯干（辅助得分）",
            "得分区: 躯干护甲",
            "头击: 禁止（安全考虑）",
            "转身踢: 允许加分",
            "比赛时间: 3局×2分钟",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "越出边界",
            "倒地",
            "消极比赛",
            "头击",
            "推人",
            "攻击禁击部位",
            "不当行为",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "上肢残疾分级",
            "踢击为主要技术",
            "拳打限制（上肢功能）",
            "护具适配",
            "裁判手势信号",
            "比赛节奏调整",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "WP分级认证",
            "最低残疾标准",
            "WT注册",
            "体重认证",
            "医疗证明",
        ]
    }
}

impl Default for ParaTaekwondoRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaTaekwondoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_taekwondo")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人跆拳道规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            装备要求:\n{}\n\n\
            技术规则:\n{}",
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
            self.technique()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_para_taekwondo_rules_basic() {
        let rules = ParaTaekwondoRules::new();
        assert_eq!(rules.metadata().name, "残疾人跆拳道规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_taekwondo_classification() {
        let rules = ParaTaekwondoRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("K41")));
        assert!(classification.iter().any(|c| c.contains("K44")));
        assert!(classification.iter().any(|c| c.contains("上肢")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_taekwondo_events() {
        let rules = ParaTaekwondoRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("残奥会")));
        assert!(events.iter().any(|e| e.contains("世界锦标赛")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_taekwondo_technique() {
        let rules = ParaTaekwondoRules::new();
        let technique = rules.technique();
        assert!(technique.iter().any(|t| t.contains("踢击")));
        assert!(technique.iter().any(|t| t.contains("得分")));
        assert!(technique.len() >= 4);
    }

    #[test]
    fn test_para_taekwondo_category() {
        let rules = ParaTaekwondoRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
