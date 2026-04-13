//! 手球详细规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 手球详细规则
pub struct HandballDetailedRules {
    metadata: RuleMetadata,
}

impl HandballDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "手球详细规则",
                "手球比赛详细规则"
            )
            .with_origin("德国")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 2半场",
            "每半场30分钟",
            "中场休息15分钟",
            "加时规则",
            "比赛结束",
        ]
    }

    /// 场地规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 40×20米",
            "球门尺寸: 2×3米",
            "禁区: 6米半径",
            "自由投掷线: 9米",
            "场地布置",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队7人上场",
            "1名守门员",
            "6名场上球员",
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
            "步法规则",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "持球超过3秒",
            "走步超过3步",
            "非法防守",
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
            "手球",
            "比赛服装",
            "守门员装备",
            "防护装备",
            "球鞋",
        ]
    }
}

impl Default for HandballDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HandballDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("handball_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【手球详细规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.court_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
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
    fn test_handball_detailed_rules() {
        let rules = HandballDetailedRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}