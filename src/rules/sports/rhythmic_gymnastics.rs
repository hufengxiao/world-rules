//! 艺术体操规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 艺术体操规则
pub struct RhythmicGymnasticsRules {
    metadata: RuleMetadata,
}

impl RhythmicGymnasticsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "艺术体操规则",
                "艺术体操比赛基本规则"
            )
            .with_origin("苏联")
            .with_tags(vec!["体育".into(), "体操".into()]),
        }
    }

    /// 比赛项目
    pub fn apparatus_types(&self) -> Vec<&'static str> {
        vec![
            "绳操",
            "圈操",
            "球操",
            "棒操",
            "带操",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "个人全能赛",
            "个人单项赛",
            "团体赛",
            "集体项目",
            "世界锦标赛",
        ]
    }

    /// 动作要求
    pub fn routine_requirements(&self) -> Vec<&'static str> {
        vec![
            "器械技术动作",
            "身体难度动作",
            "舞蹈动作",
            "动作连贯性",
            "音乐配合",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "难度分数: D分",
            "执行分数: E分",
            "艺术分数",
            "扣分项目",
            "总分计算",
        ]
    }

    /// 难度动作
    pub fn difficulty_elements(&self) -> Vec<&'static str> {
        vec![
            "跳跃动作",
            "平衡动作",
            "旋转动作",
            "柔韧动作",
            "器械操作",
        ]
    }

    /// 比赛时间
    pub fn time_limits(&self) -> Vec<&'static str> {
        vec![
            "个人项目: 1分15秒-1分30秒",
            "团体项目: 2分15秒-2分30秒",
            "音乐同步",
            "时间扣分",
            "计时精确",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "体操服",
            "器械规格标准",
            "音乐伴奏",
            "场地13×13米",
            "无鞋比赛",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "器械掉落",
            "超出场地",
            "时间超限",
            "动作违规",
            "服装违规",
        ]
    }
}

impl Default for RhythmicGymnasticsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RhythmicGymnasticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("rhythmic_gymnastics")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【艺术体操规则】\n\n\
            比赛项目:\n{}\n\n\
            评分标准:\n{}\n\n\
            动作要求:\n{}\n\n\
            犯规规则:\n{}\n",
            self.apparatus_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.routine_requirements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rhythmic_gymnastics_rules() {
        let rules = RhythmicGymnasticsRules::new();
        assert!(!rules.apparatus_types().is_empty());
    }
}