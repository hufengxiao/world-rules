//! 调查问卷设计
//!
//! 设计科学调查问卷、抽样与分析的原则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SurveyDesignRules,
    name: "调查问卷设计",
    desc: "设计科学调查问卷、抽样与分析的原则",
    origin: "国际",
    tags: ["科学", "调查", "问卷", "抽样", "统计"]
}

impl SurveyDesignRules {
    /// 明确目标
    pub fn objective(&self) -> Vec<&'static str> {
        vec![
            "明确调查的问题与对象",
            "确定测量的概念",
            "设计可操作的题项",
            "预先设定分析方向",
        ]
    }

    /// 题项设计
    pub fn items(&self) -> Vec<&'static str> {
        vec![
            "用清晰易懂的表述",
            "避免诱导或双重问题",
            "选项互斥且齐全",
            "题目用语中性",
        ]
    }

    /// 抽样方法
    pub fn sampling(&self) -> Vec<&'static str> {
        vec![
            "采用具有代表性的抽样",
            "界定目标总体",
            "样本量合适",
            "说明抽样方法",
        ]
    }

    /// 结果分析
    pub fn analysis(&self) -> Vec<&'static str> {
        vec![
            "描述样本特征",
            "关注回复率与偏差",
            "审慎外推总体",
            "如实呈现限制与不确定性",
        ]
    }
}

impl Rule for SurveyDesignRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("survey_design")
    }

    fn explain(&self) -> String {
        format!(
            "【调查问卷设计】\n{}",
            [
                format!(
                    "明确目标：\\n{}",
                    self.objective()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "题项设计：\\n{}",
                    self.items()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "抽样方法：\\n{}",
                    self.sampling()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结果分析：\\n{}",
                    self.analysis()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_surveydesignrules_basic() {
        let rules = SurveyDesignRules::new();
        assert_eq!(rules.metadata().name, "调查问卷设计");
        assert!(!rules.objective().is_empty());
        assert!(!rules.items().is_empty());
        assert!(!rules.sampling().is_empty());
        assert!(!rules.analysis().is_empty());
    }

    #[test]
    fn test_surveydesignrules_validation() {
        let rules = SurveyDesignRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("survey_design"));
    }

    #[test]
    fn test_surveydesignrules_explain() {
        let rules = SurveyDesignRules::new();
        let e = rules.explain();
        assert!(e.contains("明确目标"));
        assert!(e.contains("题项设计"));
        assert!(e.contains("抽样方法"));
    }
}
