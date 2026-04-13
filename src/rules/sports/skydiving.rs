//! 跳伞规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 跳伞规则 (Skydiving)
pub struct SkydivingRules {
    metadata: RuleMetadata,
}

impl SkydivingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "跳伞规则",
                "跳伞运动规则"
            )
            .with_origin("法国")
            .with_tags(vec!["体育".into(), "航空".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "定点跳伞",
            "花样跳伞",
            "造型跳伞",
            "翼装跳伞",
            "团体跳伞",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛高度",
            "时间限制",
            "评分标准",
            "裁判评分",
            "安全规则",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "跳伞技术",
            "自由落体控制",
            "开伞技术",
            "着陆技术",
            "花样动作",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "双伞系统",
            "高度限制",
            "天气条件",
            "装备检查",
            "医疗支持",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "主降落伞",
            "备用降落伞",
            "跳伞服装",
            "高度计",
            "头盔",
        ]
    }

    /// 级别体系
    pub fn skill_levels(&self) -> Vec<&'static str> {
        vec![
            "初级跳伞员",
            "中级跳伞员",
            "高级跳伞员",
            "教练级别",
            "专业级别",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "定点精度",
            "花样难度",
            "执行质量",
            "时间控制",
            "团队配合",
        ]
    }
}

impl Default for SkydivingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SkydivingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("skydiving")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【跳伞规则】\n\n\
            比赛类型:\n{}\n\n\
            技术动作:\n{}\n\n\
            安全规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skydiving_rules() {
        let rules = SkydivingRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}