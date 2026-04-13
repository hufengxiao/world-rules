//! 板球详细规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 板球详细规则
pub struct CricketDetailedRules {
    metadata: RuleMetadata,
}

impl CricketDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "板球详细规则",
                "板球比赛详细规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            " Test比赛: 5天",
            " ODI比赛: 50回合",
            " T20比赛: 20回合",
            "比赛分类",
            "比赛规定",
        ]
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 椭圆形",
            "球场长度: 20.12米",
            "球场宽度: 3.05米",
            "边界距离",
            "场地布置",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队11人上场",
            "击球手",
            "投球手",
            "守备员",
            "替补队员",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "击球技术",
            "投球技术",
            "守备技术",
            "跑垒技术",
            "战术运用",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "跑垒得分",
            "边界得分: 4分或6分",
            "额外得分",
            "得分统计",
            "比分记录",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "投球犯规",
            "击球犯规",
            "守备犯规",
            "犯规处罚",
            "违规判定",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "板球棒",
            "板球",
            "防护装备",
            "比赛服装",
            "手套护具",
        ]
    }
}

impl Default for CricketDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CricketDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("cricket_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【板球详细规则】\n\n\
            比赛类型:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
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
    fn test_cricket_detailed_rules() {
        let rules = CricketDetailedRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}