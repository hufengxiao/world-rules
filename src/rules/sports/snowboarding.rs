//! 单板滑雪规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 单板滑雪规则
pub struct SnowboardingRules {
    metadata: RuleMetadata,
}

impl SnowboardingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "单板滑雪规则",
                "单板滑雪比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "平行大回转: 竞速比赛",
            "障碍追逐: 越野比赛",
            "U型场地: 技术表演",
            "坡面障碍: 障碍技术",
            "大跳台: 跳跃比赛",
        ]
    }

    /// 平行大回转规则
    pub fn parallel_giant_slalom_rules(&self) -> Vec<&'static str> {
        vec![
            "两人同时比赛",
            "穿越旗门",
            "速度决定胜负",
            "违规罚时",
            "淘汰赛制",
        ]
    }

    /// U型场地规则
    pub fn halfpipe_rules(&self) -> Vec<&'static str> {
        vec![
            "U型场地表演",
            "高度和技术",
            "评分制",
            "动作多样性",
            "流畅度",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "难度: 动作难度",
            "执行: 完成质量",
            "幅度: 飞行高度",
            "流畅度: 连续性",
            "创新: 新技术",
        ]
    }

    /// 技术动作
    pub fn tricks(&self) -> Vec<&'static str> {
        vec![
            "旋转: 多周旋转",
            "翻转: 空中翻转",
            "抓板: 各种抓板",
            "组合动作",
            "着陆技术",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "单板: 单块滑雪板",
            "固定器: 固定双脚",
            "滑雪靴: 柔软靴子",
            "头盔: 安全保护",
            "护具: 保护装备",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "头盔必须佩戴",
            "场地安全检查",
            "天气条件监控",
            "医疗支持",
            "训练要求",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "预赛和决赛",
            "淘汰赛制",
            "世界杯系列",
            "世界锦标赛",
            "奥运会项目",
        ]
    }
}

impl Default for SnowboardingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SnowboardingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("snowboarding")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【单板滑雪规则】\n\n\
            比赛项目:\n{}\n\n\
            评分标准:\n{}\n\n\
            技术动作:\n{}\n\n\
            安全规则:\n{}\n",
            self.competition_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.tricks().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snowboarding_rules() {
        let rules = SnowboardingRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}