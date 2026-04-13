//! 帆板运动规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 帆板运动规则 ( Windsurfing)
pub struct WindsurfingRules {
    metadata: RuleMetadata,
}

impl WindsurfingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "帆板运动规则",
                "帆板冲浪运动规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "竞速比赛",
            "花样比赛",
            "大浪比赛",
            "自由式比赛",
            "长距离比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛路线",
            "时间限制",
            "评分标准",
            "风力要求",
            "安全规则",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "基础操控",
            "转向技巧",
            "跳跃动作",
            "花样技巧",
            "竞速技术",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "完成时间",
            "技术难度",
            "执行质量",
            "创新表现",
            "流畅性",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "安全区域",
            "风力限制",
            "救生装备",
            "救援准备",
            "医疗支持",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "帆板装备",
            "帆具系统",
            "桅杆横杆",
            "防护装备",
            "安全装备",
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

impl Default for WindsurfingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WindsurfingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("windsurfing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【帆板运动规则】\n\n\
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
    fn test_windsurfing_rules() {
        let rules = WindsurfingRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}