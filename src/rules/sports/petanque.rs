//! 法式滚球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 法式滚球规则 (Pétanque)
pub struct PetanqueRules {
    metadata: RuleMetadata,
}

impl PetanqueRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "法式滚球规则",
                "法国滚球运动规则"
            )
            .with_origin("法国")
            .with_tags(vec!["体育".into(), "休闲".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛采用局数制",
            "每局最多13分",
            "投掷规则",
            "得分规则",
            "比赛结束",
        ]
    }

    /// 场地规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 15×4米",
            "目标球位置",
            "投掷区域",
            "场地边界",
            "表面要求",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "投掷技术",
            "定点投掷",
            "击球技术",
            "瞄准技术",
            "控制技术",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "距离目标球最近得分",
            "每局得分上限",
            "得分测量",
            "比分记录",
            "比赛胜负",
        ]
    }

    /// 球的规格
    pub fn ball_specifications(&self) -> Vec<&'static str> {
        vec![
            "大球直径: 70.5-80毫米",
            "目标球直径: 30毫米",
            "重量: 650-800克",
            "材质要求",
            "颜色区分",
        ]
    }

    /// 团队配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "单人比赛: 各3球",
            "双人比赛: 各3球",
            "三人比赛: 各2球",
            "队员轮换",
            "比赛顺序",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "滚球",
            "目标球",
            "测量工具",
            "场地装备",
            "比赛服装",
        ]
    }
}

impl Default for PetanqueRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PetanqueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("petanque")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【法式滚球规则】\n\n\
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
    fn test_petanque_rules() {
        let rules = PetanqueRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}