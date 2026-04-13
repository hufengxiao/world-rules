//! 风筝冲浪规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 风筝冲浪规则
pub struct KitesurfingRules {
    metadata: RuleMetadata,
}

impl KitesurfingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "风筝冲浪规则",
                "风筝冲浪运动规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "自由式比赛",
            "竞速比赛",
            "大浪比赛",
            "长距离比赛",
            "花样比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间限制",
            "评分标准",
            "风力要求",
            "安全规则",
            "比赛区域",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "基础操控",
            "跳跃动作",
            "旋转技巧",
            "花样动作",
            "竞速技巧",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "技术难度",
            "执行质量",
            "创新表现",
            "高度幅度",
            "流畅性",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "安全释放系统",
            "头盔佩戴",
            "安全区域",
            "风力限制",
            "救援准备",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "风筝装备",
            "冲浪板",
            "控制杆",
            "安全系统",
            "防护装备",
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

impl Default for KitesurfingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for KitesurfingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("kitesurfing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【风筝冲浪规则】\n\n\
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
    fn test_kitesurfing_rules() {
        let rules = KitesurfingRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}