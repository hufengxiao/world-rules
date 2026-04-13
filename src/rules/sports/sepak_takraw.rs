//! 藤球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 藤球规则 (Sepak Takraw)
pub struct SepakTakrawRules {
    metadata: RuleMetadata,
}

impl SepakTakrawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "藤球规则",
                "东南亚藤球运动规则"
            )
            .with_origin("东南亚")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "三人藤球",
            "双人藤球",
            "团体藤球",
            "花样藤球",
            "比赛分类",
        ]
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
            "场地尺寸: 13.4×6.1米",
            "球网高度: 1.52米(男)",
            "球网高度: 1.42米(女)",
            "场地划分",
            "安全区域",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "三人队伍",
            "场上位置",
            "发球员",
            "防守员",
            "网前员",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "踢球技术",
            "头球技术",
            "控球技术",
            "射门技术",
            "防守技术",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "球落地得分",
            "对方犯规得分",
            "得分统计",
            "比分记录",
            "胜负判定",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "藤球",
            "比赛服装",
            "运动鞋",
            "护膝护腕",
            "场地装备",
        ]
    }
}

impl Default for SepakTakrawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SepakTakrawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sepak_takraw")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【藤球规则】\n\n\
            比赛类型:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
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
    fn test_sepak_takraw_rules() {
        let rules = SepakTakrawRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}