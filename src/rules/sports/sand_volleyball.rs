//! 沙地排球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 沙地排球规则 (Sand Volleyball)
pub struct SandVolleyballRules {
    metadata: RuleMetadata,
}

impl SandVolleyballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "沙地排球规则",
                "沙地排球比赛规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛采用21分制",
            "三局两胜",
            "每局最高25分",
            "发球规则",
            "换边规则",
        ]
    }

    /// 场地规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 16×8米",
            "沙地深度: 40厘米",
            "球网高度: 2.43米(男)",
            "球网高度: 2.24米(女)",
            "场地布置",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队2人上场",
            "场上位置",
            "无替补",
            "队员轮换",
            "配合规则",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "发球技术",
            "传球技术",
            "扣球技术",
            "防守技术",
            "沙滩技术",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "触网犯规",
            "过线犯规",
            "连击犯规",
            "犯规处罚",
            "违规判定",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "球落地得分",
            "有效得分",
            "得分统计",
            "比分记录",
            "胜负判定",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "排球",
            "比赛服装",
            "防护装备",
            "场地装备",
            "附属配件",
        ]
    }
}

impl Default for SandVolleyballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SandVolleyballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sand_volleyball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【沙地排球规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.court_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
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
    fn test_sand_volleyball_rules() {
        let rules = SandVolleyballRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}