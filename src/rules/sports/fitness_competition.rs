//! 健身比赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 健身比赛规则
pub struct FitnessCompetitionRules {
    metadata: RuleMetadata,
}

impl FitnessCompetitionRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "健身比赛规则",
                "健身比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "健身".into()]),
        }
    }

    /// 比赛分类
    pub fn competition_categories(&self) -> Vec<&'static str> {
        vec![
            "健身小姐",
            "健身先生",
            "健身模特",
            "肌肉模特",
            "健康模特",
        ]
    }

    /// 比赛轮次
    pub fn competition_rounds(&self) -> Vec<&'static str> {
        vec![
            "泳装轮",
            "晚装轮",
            "运动装轮",
            "体能挑战轮",
            "综合评分",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "身材比例",
            "肌肉线条",
            "皮肤状况",
            "舞台表现",
            "整体形象",
        ]
    }

    /// 体能测试
    pub fn fitness_tests(&self) -> Vec<&'static str> {
        vec![
            "力量测试",
            "柔韧测试",
            "耐力测试",
            "协调测试",
            "体能挑战",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "比赛泳装",
            "晚装要求",
            "运动装要求",
            "高跟鞋规定",
            "妆容限制",
        ]
    }

    /// 准备规则
    pub fn preparation_rules(&self) -> Vec<&'static str> {
        vec![
            "赛前训练",
            "饮食控制",
            "皮肤护理",
            "妆容准备",
            "形象设计",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "使用违禁药物",
            "过度化妆",
            "不当行为",
            "干扰比赛",
            "违反规定",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "预赛选拔",
            "决赛评选",
            "国际比赛",
            "国内比赛",
            "地区比赛",
        ]
    }
}

impl Default for FitnessCompetitionRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FitnessCompetitionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("fitness_competition")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【健身比赛规则】\n\n\
            比赛轮次:\n{}\n\n\
            评分标准:\n{}\n\n\
            体能测试:\n{}\n\n\
            禁止行为:\n{}\n",
            self.competition_rounds().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fitness_tests().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fitness_competition_rules() {
        let rules = FitnessCompetitionRules::new();
        assert!(!rules.competition_rounds().is_empty());
    }
}