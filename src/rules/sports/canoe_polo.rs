//! 独木舟 Polo规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 独木舟 Polo规则
pub struct CanoePoloRules {
    metadata: RuleMetadata,
}

impl CanoePoloRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "独木舟 Polo规则",
                "独木舟 Polo比赛规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 2半场",
            "每半场10分钟",
            "有效时间制",
            "中场休息",
            "加时赛规则",
        ]
    }

    /// 场地规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 35×23米",
            "球门高度: 1.5米",
            "水面要求",
            "边界规定",
            "安全区域",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队5人上场",
            "替补队员",
            "场上位置",
            "换人规则",
            "队员轮换",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "划船技术",
            "持球技术",
            "传球技术",
            "射门技术",
            "防守技术",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "危险动作",
            "非法接触",
            "违规持球",
            "阻挡犯规",
            "犯规处罚",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "独木舟",
            "桨",
            "防护头盔",
            "防护面罩",
            "比赛服装",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "球进入球门",
            "有效进球",
            "得分统计",
            "比分记录",
            "胜负判定",
        ]
    }
}

impl Default for CanoePoloRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CanoePoloRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("canoe_polo")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【独木舟 Polo规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            犯规规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.court_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
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
    fn test_canoe_polo_rules() {
        let rules = CanoePoloRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}