//! 长曲棍球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 长曲棍球规则 (Lacrosse)
pub struct LacrosseRules {
    metadata: RuleMetadata,
}

impl LacrosseRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "长曲棍球规则",
                "长曲棍球比赛规则"
            )
            .with_origin("北美原住民")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "场地长曲棍球",
            "箱式长曲棍球",
            "女子长曲棍球",
            "男子长曲棍球",
            "室内长曲棍球",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 4节",
            "每节15分钟",
            "中场休息",
            "暂停规则",
            "加时赛规则",
        ]
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 110×60米",
            "球门尺寸: 1.83×1.83米",
            "进攻区域",
            "防守区域",
            "中场区域",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队10人上场(场地)",
            "每队6人上场(箱式)",
            "1名守门员",
            "场上球员",
            "替补队员",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "持球技术",
            "传球技术",
            "射门技术",
            "防守技术",
            "跑位技术",
        ]
    }

    /// 犯规规则
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "技术犯规",
            "个人犯规",
            "罚时规则",
            "罚下场",
            "犯规处罚",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "长曲棍球杆",
            "防护头盔",
            "手套护具",
            "比赛服装",
            "守门员装备",
        ]
    }
}

impl Default for LacrosseRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for LacrosseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("lacrosse")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【长曲棍球规则】\n\n\
            比赛类型:\n{}\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.field_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lacrosse_rules() {
        let rules = LacrosseRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}