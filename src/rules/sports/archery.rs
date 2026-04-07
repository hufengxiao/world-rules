//! 射箭规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 射箭规则
pub struct ArcheryRules {
    metadata: RuleMetadata,
}

impl ArcheryRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "射箭规则",
                "WA 世界射箭联合会标准规则"
            )
            .with_origin("WA")
            .with_tags(vec!["体育".into(), "射箭".into()]),
        }
    }

    /// 比赛距离
    pub fn competition_distances(&self) -> Vec<u16> {
        vec![30, 50, 60, 70, 90] // 米
    }

    /// 奥运项目
    pub fn olympic_events(&self) -> Vec<&'static str> {
        vec![
            "个人排名赛: 72箭(70米)",
            "个人淘汰赛: 对抗制",
            "团体赛: 3人团体",
            "混合团体赛: 1男1女",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "靶面: 1-10环",
            "内10环(X)用于同分判定",
            "每局3箭(个人)或6箭(团体)",
            "总分高的获胜",
        ]
    }

    /// 箭支规格
    pub fn arrow_specifications(&self) -> Vec<&'static str> {
        vec![
            "最大直径: 9.3毫米",
            "箭杆材料: 碳纤维、铝合金等",
            "箭羽: 塑料或羽毛",
            "箭头: 锥形或子弹形",
        ]
    }

    /// 弓的类型
    pub fn bow_types(&self) -> Vec<&'static str> {
        vec![
            "反曲弓(奥运项目)",
            "复合弓",
            "传统弓",
            "光弓",
        ]
    }

    /// 时间限制
    pub fn time_limits(&self) -> Vec<&'static str> {
        vec![
            "每组3箭: 2分钟",
            "每组6箭: 4分钟",
            "个人淘汰赛: 每方20秒/箭",
            "超时: 该箭不计分",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "超时射箭",
            "越线射箭",
            "使用非法装备",
            "干扰其他选手",
        ]
    }
}

impl Default for ArcheryRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ArcheryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("archery")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【射箭规则】\n\n\
            奥运项目:\n{}\n\n\
            计分规则:\n{}\n\n\
            弓的类型:\n{}\n\n\
            时间限制:\n{}\n",
            self.olympic_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.bow_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.time_limits().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archery_rules() {
        let rules = ArcheryRules::new();
        assert!(!rules.olympic_events().is_empty());
    }
}