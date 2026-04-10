//! 滑板规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 滑板规则
pub struct SkateboardingRules {
    metadata: RuleMetadata,
}

impl SkateboardingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "滑板规则",
                "滑板比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "极限".into()]),
        }
    }

    /// 比赛项目
    pub fn disciplines(&self) -> Vec<&'static str> {
        vec![
            "街式滑板: 城市障碍",
            "碗池滑板: 泳池场地",
            "垂直滑板: U型场地",
            "大跳台: 高难度动作",
            "平地花式: 地面技巧",
        ]
    }

    /// 街式滑板规则
    pub fn street_rules(&self) -> Vec<&'static str> {
        vec![
            "障碍物: 阶梯、栏杆、坡道",
            "限时比赛",
            "动作多样性评分",
            "创新技术加分",
            "落地质量评分",
        ]
    }

    /// 碗池滑板规则
    pub fn park_rules(&self) -> Vec<&'static str> {
        vec![
            "碗池场地",
            "连续滑行",
            "高度和技术",
            "流畅度评分",
            "多次尝试",
        ]
    }

    /// 得分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "难度: 技术挑战度",
            "执行: 完成质量",
            "风格: 个人特色",
            "多样性: 动作变化",
            "创新: 新技术",
        ]
    }

    /// 计分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "每轮0-100分",
            "5名裁判评分",
            "去掉最高最低分",
            "取平均分",
            "最高两轮成绩",
        ]
    }

    /// 技术动作
    pub fn tricks(&self) -> Vec<&'static str> {
        vec![
            "跳跃动作: Ollie、Kickflip",
            "旋转动作: 360度旋转",
            "抓板动作: 各种抓板",
            "滑行动作: 滑栏杆",
            "翻转动作: 空中翻转",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "滑板: 标准尺寸",
            "轮子: 不同硬度",
            "轴承: 速度控制",
            "头盔: 安全保护",
            "护具: 膝盖和肘部",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "大型比赛佩戴头盔",
            "场地安全检查",
            "医疗支持在场",
            "比赛监督",
            "危险动作警告",
        ]
    }
}

impl Default for SkateboardingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SkateboardingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("skateboarding")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【滑板规则】\n\n\
            比赛项目:\n{}\n\n\
            得分标准:\n{}\n\n\
            技术动作:\n{}\n\n\
            装备要求:\n{}\n",
            self.disciplines().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.tricks().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skateboarding_rules() {
        let rules = SkateboardingRules::new();
        assert!(!rules.disciplines().is_empty());
    }
}