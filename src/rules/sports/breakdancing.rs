//! 街舞规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 街舞规则 (Breakdancing)
pub struct BreakdancingRules {
    metadata: RuleMetadata,
}

impl BreakdancingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "街舞规则",
                "街舞比赛规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "舞蹈".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "单人比赛",
            "团队比赛",
            "Battle比赛",
            "表演比赛",
            "综合比赛",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "Toprock站立舞步",
            "Downrock地板舞步",
            "Power Move力量动作",
            "Freeze定格动作",
            "Transition转换动作",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "技术难度",
            "创意表现",
            "音乐诠释",
            "执行质量",
            "整体表现",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间限制",
            "轮次规则",
            "服装规定",
            "音乐规则",
            "裁判评分",
        ]
    }

    /// Battle规则
    pub fn battle_rules(&self) -> Vec<&'static str> {
        vec![
            "1对1对决",
            "回合制",
            "轮流表演",
            "裁判判定",
            "胜负规则",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "比赛服装",
            "运动鞋",
            "护膝护腕",
            "场地装备",
            "音乐设备",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "场地安全",
            "护具佩戴",
            "动作控制",
            "医疗支持",
            "比赛控制",
        ]
    }
}

impl Default for BreakdancingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BreakdancingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("breakdancing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【街舞规则】\n\n\
            技术动作:\n{}\n\n\
            评分标准:\n{}\n\n\
            Battle规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.battle_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakdancing_rules() {
        let rules = BreakdancingRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}