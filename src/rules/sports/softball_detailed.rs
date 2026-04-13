//! 垒球详细规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 垒球详细规则
pub struct SoftballDetailedRules {
    metadata: RuleMetadata,
}

impl SoftballDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "垒球详细规则",
                "垒球比赛详细规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "快投垒球",
            "慢投垒球",
            "男女比赛",
            "青少年比赛",
            "比赛分类",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛局数: 7局",
            "每局得分",
            "时间限制",
            "加局规则",
            "比赛结束",
        ]
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "垒间距: 60英尺",
            "投球距离: 43英尺(快投)",
            "投球距离: 50英尺(慢投)",
            "场地尺寸",
            "安全区域",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队9人上场",
            "场上位置",
            "替补队员",
            "换人规则",
            "教练指导",
        ]
    }

    /// 投球规则
    pub fn pitching_rules(&self) -> Vec<&'static str> {
        vec![
            "投球方式",
            "投球速度限制",
            "投球高度(慢投)",
            "投球犯规",
            "投球计数",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "跑垒得分",
            "有效得分",
            "得分统计",
            "比分记录",
            "胜负判定",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "垒球",
            "垒球棒",
            "手套",
            "比赛服装",
            "防护装备",
        ]
    }
}

impl Default for SoftballDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SoftballDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("softball_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【垒球详细规则】\n\n\
            场地规格:\n{}\n\n\
            投球规则:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.field_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.pitching_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_softball_detailed_rules() {
        let rules = SoftballDetailedRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}