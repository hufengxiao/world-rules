//! 美式足球详细规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 美式足球详细规则
pub struct AmericanFootballDetailedRules {
    metadata: RuleMetadata,
}

impl AmericanFootballDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "美式足球详细规则",
                "美式足球比赛详细规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 4节",
            "每节15分钟",
            "有效时间制",
            "中场休息",
            "加时规则",
        ]
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地长度: 100码",
            "场地宽度: 53.3码",
            "得分区: 10码深度",
            "球场标记",
            "安全区域",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队11人上场",
            "进攻阵容",
            "防守阵容",
            "特勤阵容",
            "替补队员",
        ]
    }

    /// 进攻规则
    pub fn offense_rules(&self) -> Vec<&'static str> {
        vec![
            "四次进攻机会",
            "推进10码重新获得",
            "传球规则",
            "跑球规则",
            "进攻战术",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            " touchdown: 6分",
            "加分射门: 1分",
            "加分跑球: 2分",
            "射门得分: 3分",
            "安全得分: 2分",
        ]
    }

    /// 犯规规则
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "非法阻挡",
            "持球犯规",
            "越位犯规",
            "犯规处罚",
            "罚码规则",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "美式足球",
            "防护头盔",
            "护具套装",
            "比赛服装",
            "球鞋",
        ]
    }
}

impl Default for AmericanFootballDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AmericanFootballDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("american_football_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【美式足球详细规则】\n\n\
            场地规格:\n{}\n\n\
            进攻规则:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.field_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.offense_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_american_football_detailed_rules() {
        let rules = AmericanFootballDetailedRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}