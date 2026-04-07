//! 体操规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 体操项目类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GymnasticsType {
    /// 竞技体操
    Artistic,
    /// 艺术体操
    Rhythmic,
    /// 蹦床
    Trampoline,
}

impl GymnasticsType {
    pub fn name(&self) -> &'static str {
        match self {
            GymnasticsType::Artistic => "竞技体操",
            GymnasticsType::Rhythmic => "艺术体操",
            GymnasticsType::Trampoline => "蹦床",
        }
    }
}

/// 体操规则
pub struct GymnasticsRules {
    metadata: RuleMetadata,
}

impl GymnasticsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "体操规则",
                "FIG 国际体操联合会标准规则"
            )
            .with_origin("FIG")
            .with_tags(vec!["体育".into(), "体操".into()]),
        }
    }

    /// 男子竞技体操项目
    pub fn men_apparatus(&self) -> Vec<&'static str> {
        vec![
            "自由体操",
            "鞍马",
            "吊环",
            "跳马",
            "双杠",
            "单杠",
        ]
    }

    /// 女子竞技体操项目
    pub fn women_apparatus(&self) -> Vec<&'static str> {
        vec![
            "跳马",
            "高低杠",
            "平衡木",
            "自由体操",
        ]
    }

    /// 艺术体操器械
    pub fn rhythmic_apparatus(&self) -> Vec<&'static str> {
        vec![
            "绳",
            "圈",
            "球",
            "棒",
            "带",
        ]
    }

    /// 评分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "难度分(D分): 动作难度价值",
            "完成分(E分): 动作完成质量",
            "总分 = D分 + E分 - 扣分",
            "满分理论上无上限",
        ]
    }

    /// 扣分项目
    pub fn deductions(&self) -> Vec<&'static str> {
        vec![
            "动作失误",
            "落地不稳",
            "出界",
            "时间违规",
            "服装违规",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "团体赛: 5-3-3制(5人报名，3人上场，3人计分)",
            "个人全能: 所有项目分数相加",
            "单项决赛: 各项目前8名",
        ]
    }
}

impl Default for GymnasticsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GymnasticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("gymnastics")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【体操规则】\n\n\
            男子项目:\n{}\n\n\
            女子项目:\n{}\n\n\
            评分系统:\n{}\n\n\
            比赛形式:\n{}\n",
            self.men_apparatus().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.women_apparatus().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_formats().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gymnastics_rules() {
        let rules = GymnasticsRules::new();
        assert_eq!(rules.men_apparatus().len(), 6);
        assert_eq!(rules.women_apparatus().len(), 4);
    }
}