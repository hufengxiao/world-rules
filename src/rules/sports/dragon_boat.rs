//! 龙舟比赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 龙舟比赛规则
pub struct DragonBoatRules {
    metadata: RuleMetadata,
}

impl DragonBoatRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "龙舟比赛规则",
                "中国传统龙舟竞渡规则"
            )
            .with_origin("中国")
            .with_tags(vec!["体育".into(), "水上".into(), "传统".into()]),
        }
    }

    /// 船艇规格
    pub fn boat_specifications(&self) -> Vec<&'static str> {
        vec![
            "标准龙舟: 12.4米",
            "小龙舟: 8.6米",
            "标准龙舟22人",
            "小龙舟12人",
            "龙头龙尾装饰",
        ]
    }

    /// 人员配置
    pub fn crew_composition(&self) -> Vec<&'static str> {
        vec![
            "划手配置",
            "鼓手指挥",
            "舵手控制",
            "替补队员",
            "团队协作",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛距离: 200-1000米",
            "直线航道",
            "同时出发",
            "终点判定",
            "犯规规则",
        ]
    }

    /// 技术要求
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "划桨技术",
            "配合节奏",
            "鼓点指挥",
            "舵手控制",
            "起航技术",
        ]
    }

    /// 比赛分类
    pub fn competition_categories(&self) -> Vec<&'static str> {
        vec![
            "公开组",
            "女子组",
            "混合组",
            "青少年组",
            "企业组",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "救生衣佩戴",
            "水域安全",
            "船只检查",
            "救援准备",
            "医疗支持",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "龙舟装备",
            "划桨",
            "救生衣",
            "鼓手装备",
            "舵手装备",
        ]
    }
}

impl Default for DragonBoatRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DragonBoatRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("dragon_boat")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【龙舟比赛规则】\n\n\
            船艇规格:\n{}\n\n\
            人员配置:\n{}\n\n\
            比赛规则:\n{}\n\n\
            安全规则:\n{}\n",
            self.boat_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.crew_composition().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dragon_boat_rules() {
        let rules = DragonBoatRules::new();
        assert!(!rules.boat_specifications().is_empty());
    }
}