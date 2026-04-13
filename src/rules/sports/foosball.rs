//! 桌上足球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 桌上足球规则 (Foosball)
pub struct FoosballRules {
    metadata: RuleMetadata,
}

impl FoosballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "桌上足球规则",
                "桌上足球比赛规则"
            )
            .with_origin("德国")
            .with_tags(vec!["体育".into(), "休闲".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛采用5球制",
            "比赛时间限制",
            "轮流开始",
            "得分规则",
            "比赛结束",
        ]
    }

    /// 球台规格
    pub fn table_specifications(&self) -> Vec<&'static str> {
        vec![
            "球台长度: 56英寸",
            "球台宽度: 30英寸",
            "球员杆数: 8杆",
            "球员配置",
            "球台质量",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "握杆技术",
            "推拉技术",
            "射门技术",
            "传球技术",
            "防守技术",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "旋转超过360度",
            "违规触碰",
            "干扰对方",
            "超时犯规",
            "犯规处罚",
        ]
    }

    /// 球员配置
    pub fn player_setup(&self) -> Vec<&'static str> {
        vec![
            "门将杆: 1人",
            "防守杆: 2人",
            "中场杆: 5人",
            "进攻杆: 3人",
            "球员布局",
        ]
    }

    /// 团队配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "单人比赛",
            "双人比赛",
            "队员轮换",
            "位置分配",
            "比赛顺序",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "桌上足球台",
            "比赛用球",
            "操控杆",
            "计分板",
            "比赛服装",
        ]
    }
}

impl Default for FoosballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FoosballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("foosball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【桌上足球规则】\n\n\
            球台规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            犯规规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.table_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foosball_rules() {
        let rules = FoosballRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}