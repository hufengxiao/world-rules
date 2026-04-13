//! 冰壶详细规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 冰壶详细规则
pub struct CurlingDetailedRules {
    metadata: RuleMetadata,
}

impl CurlingDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "冰壶详细规则",
                "冰壶比赛详细规则"
            )
            .with_origin("苏格兰")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛局数: 10局",
            "每局投壶",
            "时间限制",
            "加局规则",
            "比赛结束",
        ]
    }

    /// 场地规格
    pub fn sheet_specifications(&self) -> Vec<&'static str> {
        vec![
            "冰道长度: 45.72米",
            "冰道宽度: 4.75米",
            "大本营半径: 1.83米",
            "冰面质量",
            "场地布置",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队4人",
            "投壶顺序",
            "擦冰队员",
            "队长指挥",
            "队员分工",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "投壶技术",
            "控制力度",
            "旋转技术",
            "擦冰技术",
            "策略运用",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "靠近中心壶得分",
            "每局得分",
            "得分测量",
            "比分记录",
            "胜负判定",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "投壶犯规",
            "擦冰犯规",
            "干扰犯规",
            "犯规处罚",
            "违规判定",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "冰壶",
            "冰壶杆",
            "比赛服装",
            "冰壶鞋",
            "擦冰刷",
        ]
    }
}

impl Default for CurlingDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CurlingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("curling_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【冰壶详细规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.sheet_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
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
    fn test_curling_detailed_rules() {
        let rules = CurlingDetailedRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}