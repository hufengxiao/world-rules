//! 旱地冰球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 旱地冰球规则 (Floorball)
pub struct FloorballRules {
    metadata: RuleMetadata,
}

impl FloorballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "旱地冰球规则",
                "室内旱地冰球比赛规则"
            )
            .with_origin("瑞典")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 3节",
            "每节20分钟",
            "有效时间制",
            "节间休息10分钟",
            "加时赛规则",
        ]
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 40×20米",
            "球门尺寸: 1.6×1.15米",
            "禁区区域",
            "场地边界",
            "安全区域",
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
            "2分钟罚时",
            "5分钟罚时",
            "罚下场",
            "罚球规则",
            "犯规类型",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "旱地冰球杆",
            "比赛用球",
            "守门员装备",
            "比赛服装",
            "防护装备",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "球进入球门",
            "有效进球",
            "得分统计",
            "助攻统计",
            "比分记录",
        ]
    }
}

impl Default for FloorballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FloorballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("floorball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【旱地冰球规则】\n\n\
            比赛规则:\n{}\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
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
    fn test_floorball_rules() {
        let rules = FloorballRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}