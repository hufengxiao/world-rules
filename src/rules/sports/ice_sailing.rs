//! 冰上帆船规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 冰上帆船规则
pub struct IceSailingRules {
    metadata: RuleMetadata,
}

impl IceSailingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "冰上帆船规则",
                "冰上帆船比赛规则"
            )
            .with_origin("北欧")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "竞速比赛",
            "花样比赛",
            "长距离比赛",
            "团体比赛",
            "综合比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛路线",
            "时间限制",
            "出发规则",
            "终点判定",
            "安全规则",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "操控技术",
            "转向技术",
            "风帆控制",
            "速度控制",
            "安全技术",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "冰上帆船",
            "风帆装备",
            "防护服装",
            "安全装备",
            "冰面装备",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "冰面安全",
            "防护服装",
            "风力限制",
            "救援准备",
            "医疗支持",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "完成时间",
            "技术评分",
            "花样评分",
            "综合评分",
            "排名规则",
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

impl Default for IceSailingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for IceSailingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ice_sailing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【冰上帆船规则】\n\n\
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
    fn test_ice_sailing_rules() {
        let rules = IceSailingRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}