//! 室内足球详细规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 室内足球详细规则 (Indoor Soccer)
pub struct IndoorSoccerRules {
    metadata: RuleMetadata,
}

impl IndoorSoccerRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "室内足球详细规则",
                "室内足球比赛详细规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 2半场",
            "每半场25分钟",
            "有效时间制",
            "中场休息15分钟",
            "加时规则",
        ]
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 56×26米",
            "球门尺寸: 2×3米",
            "场地边界",
            "安全区域",
            "场地布置",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队6人上场",
            "1名守门员",
            "5名场上球员",
            "替补队员",
            "换人规则",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "控球技术",
            "传球技术",
            "射门技术",
            "防守技术",
            "室内技术",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "普通犯规",
            "技术犯规",
            "严重犯规",
            "犯规处罚",
            "罚球规则",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "球进入球门得分",
            "有效进球",
            "得分统计",
            "比分记录",
            "胜负判定",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "室内足球",
            "比赛服装",
            "防护装备",
            "球鞋",
            "场地装备",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "场地安全",
            "防护装备",
            "医疗支持",
            "比赛控制",
            "应急处理",
        ]
    }
}

impl Default for IndoorSoccerRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for IndoorSoccerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("indoor_soccer")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【室内足球详细规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.field_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indoor_soccer_rules() {
        let rules = IndoorSoccerRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}