//! 站立式桨板规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 站立式桨板规则 (SUP)
pub struct PaddleboardingRules {
    metadata: RuleMetadata,
}

impl PaddleboardingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "站立式桨板规则",
                "站立式桨板运动规则"
            )
            .with_origin("夏威夷")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "竞速比赛",
            "花样比赛",
            "大浪比赛",
            "长距离比赛",
            "技术比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛距离",
            "时间限制",
            "评分标准",
            "风力要求",
            "安全规则",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "基础划桨",
            "转向技巧",
            "波浪技巧",
            "竞速技术",
            "花样动作",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "完成时间",
            "技术表现",
            "波浪选择",
            "花样难度",
            "流畅性",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "救生装备",
            "安全区域",
            "风力限制",
            "救援准备",
            "医疗支持",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "桨板装备",
            "划桨",
            "防护装备",
            "救生装备",
            "附属配件",
        ]
    }

    /// 级别体系
    pub fn skill_levels(&self) -> Vec<&'static str> {
        vec![
            "初学者级别",
            "中级水平",
            "高级水平",
            "专业级别",
            "教练认证",
        ]
    }
}

impl Default for PaddleboardingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PaddleboardingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("paddleboarding")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【站立式桨板规则】\n\n\
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
    fn test_paddleboarding_rules() {
        let rules = PaddleboardingRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}