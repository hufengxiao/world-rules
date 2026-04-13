//! 攀冰规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 攀冰规则
pub struct IceClimbingRules {
    metadata: RuleMetadata,
}

impl IceClimbingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "攀冰规则",
                "攀冰运动规则"
            )
            .with_origin("欧洲")
            .with_tags(vec!["体育".into(), "冬季".into(), "极限".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "速度攀冰",
            "难度攀冰",
            "人工冰壁",
            "自然冰壁",
            "综合比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间限制",
            "高度判定",
            "速度计时",
            "评分标准",
            "安全规则",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "冰镐技术",
            "冰爪技术",
            "保护技术",
            "攀登技术",
            "下降技术",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "冰镐",
            "冰爪",
            "攀冰靴",
            "保护装备",
            "头盔",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "头盔必须",
            "保护系统",
            "冰面检查",
            "医疗支持",
            "应急处理",
        ]
    }

    /// 级别体系
    pub fn difficulty_grades(&self) -> Vec<&'static str> {
        vec![
            "WI1-WI2: 初级",
            "WI3-WI4: 中级",
            "WI5-WI6: 高级",
            "WI7: 专家级",
            "难度评估",
        ]
    }

    /// 比赛评分
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "完成高度",
            "完成时间",
            "技术评分",
            "难度系数",
            "风格评分",
        ]
    }
}

impl Default for IceClimbingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for IceClimbingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ice_climbing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【攀冰规则】\n\n\
            比赛项目:\n{}\n\n\
            技术动作:\n{}\n\n\
            装备要求:\n{}\n\n\
            级别体系:\n{}\n",
            self.competition_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.difficulty_grades().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ice_climbing_rules() {
        let rules = IceClimbingRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}