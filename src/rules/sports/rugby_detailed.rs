//! 橄榄球详细规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 橄榄球详细规则 (Rugby)
pub struct RugbyDetailedRules {
    metadata: RuleMetadata,
}

impl RugbyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "橄榄球详细规则",
                "橄榄球比赛详细规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "橄榄球联盟",
            "橄榄球联盟",
            "七人橄榄球",
            "比赛分类",
            "比赛规定",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 2半场",
            "每半场40分钟",
            "中场休息10分钟",
            "加时规则",
            "比赛结束",
        ]
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 100×70米",
            "得分区: 10-22米深度",
            "场地标记",
            "安全区域",
            "场地布置",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队15人上场",
            "前锋球员",
            "后卫球员",
            "替补队员",
            "换人规则",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "传球技术",
            "跑球技术",
            "踢球技术",
            "擒抱技术",
            "战术运用",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "尝试得分: 5分",
            "转换射门: 2分",
            "罚踢射门: 3分",
            "落踢射门: 3分",
            "比分记录",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "危险擒抱",
            "非法阻挡",
            "越位犯规",
            "犯规处罚",
            "罚踢规则",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "橄榄球",
            "比赛服装",
            "护具套装",
            "球鞋",
            "防护装备",
        ]
    }
}

impl Default for RugbyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RugbyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("rugby_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【橄榄球详细规则】\n\n\
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
    fn test_rugby_detailed_rules() {
        let rules = RugbyDetailedRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}