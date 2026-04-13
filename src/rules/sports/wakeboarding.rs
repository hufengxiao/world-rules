//! 尾波滑水规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 尾波滑水规则
pub struct WakeboardingRules {
    metadata: RuleMetadata,
}

impl WakeboardingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "尾波滑水规则",
                "尾波滑水运动规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "花样比赛",
            "大动作比赛",
            "竞速比赛",
            "公园比赛",
            "自由式比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间限制",
            "动作次数限制",
            "评分标准",
            "两轮比赛",
            "最终评分",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "基础滑行",
            "跳跃动作",
            "旋转技巧",
            "抓板动作",
            "花样动作",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "技术难度",
            "执行质量",
            "幅度高度",
            "创新表现",
            "流畅性",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "头盔佩戴",
            "救生衣",
            "安全区域",
            "船只安全",
            "救援准备",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "滑水板",
            "绑定器",
            "牵引绳",
            "防护装备",
            "船只装备",
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

impl Default for WakeboardingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WakeboardingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wakeboarding")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【尾波滑水规则】\n\n\
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
    fn test_wakeboarding_rules() {
        let rules = WakeboardingRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}