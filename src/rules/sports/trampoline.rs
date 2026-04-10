//! 蹦床规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 蹦床规则
pub struct TrampolineRules {
    metadata: RuleMetadata,
}

impl TrampolineRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "蹦床规则",
                "蹦床比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "体操".into()]),
        }
    }

    /// 蹦床规格
    pub fn trampoline_specifications(&self) -> Vec<&'static str> {
        vec![
            "蹦床尺寸: 长5米×宽2.9米",
            "跳跃高度: 约1.15米",
            "网面材质: 尼龙或聚酯",
            "弹簧数量: 约120个",
            "框架稳固",
        ]
    }

    /// 比赛项目
    pub fn competition_categories(&self) -> Vec<&'static str> {
        vec![
            "个人蹦床",
            "双人蹦床",
            "同步蹦床",
            "小蹦床",
            "团体比赛",
        ]
    }

    /// 动作要求
    pub fn routine_requirements(&self) -> Vec<&'static str> {
        vec![
            "10个规定动作",
            "连续完成",
            "动作连贯",
            "高度稳定",
            "不能停顿",
        ]
    }

    /// 动作类型
    pub fn skill_types(&self) -> Vec<&'static str> {
        vec![
            "直体动作",
            "屈体动作",
            "团身动作",
            "旋转动作",
            "组合动作",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "难度分数",
            "执行分数",
            "高度分数",
            "同步分数(双人)",
            "扣分项目",
        ]
    }

    /// 比赛时间
    pub fn time_limits(&self) -> Vec<&'static str> {
        vec![
            "个人赛: 连续10个动作",
            "同步赛: 连续10个动作",
            "不能中断",
            "动作间隔",
            "高度控制",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "动作中断",
            "触碰框架",
            "落地不稳",
            "动作不足10个",
            "安全违规",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "安全垫铺设",
            "装备检查",
            "医疗支持",
            "裁判监督",
            "教练在场",
        ]
    }
}

impl Default for TrampolineRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TrampolineRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("trampoline")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【蹦床规则】\n\n\
            比赛项目:\n{}\n\n\
            动作要求:\n{}\n\n\
            评分标准:\n{}\n\n\
            犯规规则:\n{}\n",
            self.competition_categories().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.routine_requirements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trampoline_rules() {
        let rules = TrampolineRules::new();
        assert!(!rules.trampoline_specifications().is_empty());
    }
}