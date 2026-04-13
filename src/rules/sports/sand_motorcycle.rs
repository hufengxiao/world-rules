//! 沙地摩托车规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 沙地摩托车规则
pub struct SandMotorcycleRules {
    metadata: RuleMetadata,
}

impl SandMotorcycleRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "沙地摩托车规则",
                "沙地摩托车比赛规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "赛车".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "竞速比赛",
            "花样比赛",
            "障碍比赛",
            "耐力比赛",
            "自由式比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间限制",
            "赛道规定",
            "出发规则",
            "终点判定",
            "安全规则",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "操控技术",
            "沙地控制",
            "转向技术",
            "跳跃技术",
            "花样动作",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "防护头盔必须",
            "护具佩戴",
            "安全区域",
            "救援准备",
            "医疗支持",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "沙地摩托车",
            "防护头盔",
            "护具套装",
            "比赛服装",
            "安全装备",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "完成时间",
            "技术难度",
            "执行质量",
            "花样评分",
            "综合评分",
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

impl Default for SandMotorcycleRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SandMotorcycleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sand_motorcycle")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【沙地摩托车规则】\n\n\
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
    fn test_sand_motorcycle_rules() {
        let rules = SandMotorcycleRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}