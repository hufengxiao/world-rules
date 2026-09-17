//! 情绪日记
//!
//! 记录情绪、觉察诱因与促进自我调节的方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MoodDiaryRules,
    name: "情绪日记",
    desc: "记录情绪、觉察诱因与促进自我调节的方法",
    origin: "心理学",
    tags: ["健康", "情绪日记", "书写", "心理"]
}

impl MoodDiaryRules {
    /// 定期记录
    pub fn record(&self) -> Vec<&'static str> {
        vec![
            "固定时间写几句",
            "记下当时情绪",
            "伴随时长程度",
            "简单不设限",
        ]
    }

    /// 觉察诱因
    pub fn trigger(&self) -> Vec<&'static str> {
        vec![
            "回顾情绪前事件",
            "识别触发点",
            "思考身体反应",
            "发现情绪规律",
        ]
    }

    /// 表达梳理
    pub fn express(&self) -> Vec<&'static str> {
        vec!["如实写内心感受", "不压抑不评判", "梳理混乱思绪", "疏导郁结"]
    }

    /// 指导改变
    pub fn insight(&self) -> Vec<&'static str> {
        vec![
            "从记录找模式",
            "运用觉察调整",
            "感恩正面记录",
            "必要时求专业",
        ]
    }
}

impl Rule for MoodDiaryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("mood_diary")
    }

    fn explain(&self) -> String {
        format!(
            "【情绪日记】\n{}",
            [
                format!(
                    "定期记录：\\n{}",
                    self.record()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "觉察诱因：\\n{}",
                    self.trigger()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "表达梳理：\\n{}",
                    self.express()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "指导改变：\\n{}",
                    self.insight()
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
    fn test_mooddiaryrules_basic() {
        let rules = MoodDiaryRules::new();
        assert_eq!(rules.metadata().name, "情绪日记");
        assert!(!rules.record().is_empty());
        assert!(!rules.trigger().is_empty());
        assert!(!rules.express().is_empty());
        assert!(!rules.insight().is_empty());
    }

    #[test]
    fn test_mooddiaryrules_validation() {
        let rules = MoodDiaryRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("mood_diary"));
    }

    #[test]
    fn test_mooddiaryrules_explain() {
        let rules = MoodDiaryRules::new();
        let e = rules.explain();
        assert!(e.contains("定期记录"));
        assert!(e.contains("觉察诱因"));
        assert!(e.contains("表达梳理"));
    }
}
