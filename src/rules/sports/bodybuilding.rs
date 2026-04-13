//! 健美比赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 健美比赛规则
pub struct BodybuildingRules {
    metadata: RuleMetadata,
}

impl BodybuildingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "健美比赛规则",
                "健美比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "健身".into()]),
        }
    }

    /// 比赛分类
    pub fn competition_categories(&self) -> Vec<&'static str> {
        vec![
            "男子健美",
            "女子健美",
            "男子健体",
            "女子健体",
            "健身模特",
        ]
    }

    /// 体重级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "男子: 从轻量级到超重量级",
            "女子: 从轻量级到重量级",
            "体重间隔约5公斤",
            "赛前称重",
            "体重限制",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "肌肉发达程度",
            "肌肉对称性",
            "肌肉清晰度",
            "身体比例",
            "舞台表现",
        ]
    }

    /// 表演项目
    pub fn performance_rounds(&self) -> Vec<&'static str> {
        vec![
            "规定动作轮",
            "自由表演轮",
            "比较评分",
            "决赛表演",
            "选手展示",
        ]
    }

    /// 规定动作
    pub fn mandatory_poses(&self) -> Vec<&'static str> {
        vec![
            "正面双臂展",
            "正面展肩",
            "侧面展胸",
            "背面展背",
            "背面展腿",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "比赛泳装",
            "颜色规定",
            "无鞋比赛",
            "禁止装饰",
            "简约装束",
        ]
    }

    /// 准备规则
    pub fn preparation_rules(&self) -> Vec<&'static str> {
        vec![
            "赛前饮食控制",
            "水分控制",
            "训练周期",
            "禁止药物",
            "健康检查",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "使用违禁药物",
            "作弊行为",
            "不当展示",
            "干扰其他选手",
            "违反比赛规定",
        ]
    }
}

impl Default for BodybuildingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BodybuildingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bodybuilding")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【健美比赛规则】\n\n\
            评分标准:\n{}\n\n\
            规定动作:\n{}\n\n\
            装备要求:\n{}\n\n\
            禁止行为:\n{}\n",
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.mandatory_poses().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bodybuilding_rules() {
        let rules = BodybuildingRules::new();
        assert!(!rules.scoring_criteria().is_empty());
    }
}