//! 爱尔兰曲棍球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 爱尔兰曲棍球规则 (Hurling)
pub struct HurlingRules {
    metadata: RuleMetadata,
}

impl HurlingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "爱尔兰曲棍球规则",
                "爱尔兰传统曲棍球规则"
            )
            .with_origin("爱尔兰")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 2半场",
            "每半场35分钟",
            "有效时间制",
            "中场休息10分钟",
            "加时赛规则",
        ]
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 130-145×80-90米",
            "球门尺寸: H形球门",
            "得分区域",
            "场地划分",
            "安全区域",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队15人上场",
            "场上位置",
            "替补队员",
            "换人规则",
            "教练指导",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "击球技术",
            "传球技术",
            "携带球",
            "射门技术",
            "防守技术",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "进球: 3分",
            "射门得分: 1分",
            "得分统计",
            "比分记录",
            "胜负判定",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "携带球超过4步",
            "非法击球",
            "阻挡犯规",
            "危险动作",
            "犯规处罚",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "爱尔兰曲棍球杆",
            "比赛用球",
            "比赛服装",
            "防护头盔",
            "手套",
        ]
    }
}

impl Default for HurlingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HurlingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("hurling")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【爱尔兰曲棍球规则】\n\n\
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
    fn test_hurling_rules() {
        let rules = HurlingRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}