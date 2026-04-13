//! 瑜伽比赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 瑜伽比赛规则
pub struct YogaCompetitionRules {
    metadata: RuleMetadata,
}

impl YogaCompetitionRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "瑜伽比赛规则",
                "瑜伽比赛基本规则"
            )
            .with_origin("印度")
            .with_tags(vec!["体育".into(), "健身".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "体式比赛",
            "艺术瑜伽",
            "呼吸控制比赛",
            "个人表演",
            "团体表演",
        ]
    }

    /// 体式分类
    pub fn posture_categories(&self) -> Vec<&'static str> {
        vec![
            "站立姿势",
            "坐姿",
            "卧姿",
            "倒立姿势",
            "平衡姿势",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "体式准确性",
            "柔韧度",
            "力量表现",
            "平衡能力",
            "呼吸配合",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "规定体式表演",
            "自选体式表演",
            "时间限制",
            "难度要求",
            "裁判评分",
        ]
    }

    /// 难度等级
    pub fn difficulty_levels(&self) -> Vec<&'static str> {
        vec![
            "初级体式",
            "中级体式",
            "高级体式",
            "专业体式",
            "艺术体式",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "瑜伽服",
            "瑜伽垫",
            "比赛服装规定",
            "无鞋比赛",
            "简约装饰",
        ]
    }

    /// 场地要求
    pub fn field_requirements(&self) -> Vec<&'static str> {
        vec![
            "场地平整",
            "瑜伽垫铺设",
            "温度适宜",
            "光线良好",
            "音乐伴奏",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "体式安全",
            "避免过度伸展",
            "医疗支持",
            "选手状态评估",
            "比赛中止规则",
        ]
    }

    /// 瑜伽流派
    pub fn yoga_styles(&self) -> Vec<&'static str> {
        vec![
            "哈达瑜伽",
            "流瑜伽",
            "阿斯坦加瑜伽",
            "力量瑜伽",
            "艺术瑜伽",
        ]
    }
}

impl Default for YogaCompetitionRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for YogaCompetitionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("yoga_competition")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【瑜伽比赛规则】\n\n\
            体式分类:\n{}\n\n\
            评分标准:\n{}\n\n\
            安全规则:\n{}\n\n\
            瑜伽流派:\n{}\n",
            self.posture_categories().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.yoga_styles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yoga_competition_rules() {
        let rules = YogaCompetitionRules::new();
        assert!(!rules.posture_categories().is_empty());
    }
}