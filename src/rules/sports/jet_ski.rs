//! 水上摩托规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 水上摩托规则 (Jet Ski)
pub struct JetSkiRules {
    metadata: RuleMetadata,
}

impl JetSkiRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "水上摩托规则",
                "水上摩托竞速规则"
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
            "转向技术",
            "跳跃技术",
            "花样动作",
            "竞速技术",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "水上摩托",
            "防护头盔",
            "救生衣",
            "比赛服装",
            "安全装备",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "救生衣必须",
            "头盔佩戴",
            "安全区域",
            "救援准备",
            "医疗支持",
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

impl Default for JetSkiRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for JetSkiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("jet_ski")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【水上摩托规则】\n\n\
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
    fn test_jet_ski_rules() {
        let rules = JetSkiRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}