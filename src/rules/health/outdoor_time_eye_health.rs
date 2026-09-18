//! 户外活动护眼
//!
//! 户外活动对视力、防近视的作用

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OutdoorTimeEyeHealthRules,
    name: "户外活动护眼",
    desc: "户外活动对视力、防近视的作用",
    origin: "医学",
    tags: ["健康", "户外", "护眼", "近视"]
}

impl OutdoorTimeEyeHealthRules {
    /// 户外价值
    pub fn benefit(&self) -> Vec<&'static str> {
        vec!["日光照合维生素", "自然远望放松睫", "增强视力", "防近视关键"]
    }

    /// 时长建议
    pub fn duration(&self) -> Vec<&'static str> {
        vec!["每天两小时户外", "分散进行", "阳光充足时段", "持之以恒"]
    }

    /// 活动类型
    pub fn activity(&self) -> Vec<&'static str> {
        vec!["散步奔跑打球", "亲近自然", "远近交替看", "目之所及开阔"]
    }

    /// 结合生活
    pub fn habit(&self) -> Vec<&'static str> {
        vec!["放学回家先户外", "课间到室外", "兼顾学习", "劳逸结合"]
    }
}

impl Rule for OutdoorTimeEyeHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("outdoor_eye")
    }

    fn explain(&self) -> String {
        format!(
            "【户外活动护眼】\n{}",
            [
                format!(
                    "户外价值：\\n{}",
                    self.benefit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "时长建议：\\n{}",
                    self.duration()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "活动类型：\\n{}",
                    self.activity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结合生活：\\n{}",
                    self.habit()
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
    fn test_outdoortimeeyehealthrules_basic() {
        let rules = OutdoorTimeEyeHealthRules::new();
        assert_eq!(rules.metadata().name, "户外活动护眼");
        assert!(!rules.benefit().is_empty());
        assert!(!rules.duration().is_empty());
        assert!(!rules.activity().is_empty());
        assert!(!rules.habit().is_empty());
    }

    #[test]
    fn test_outdoortimeeyehealthrules_validation() {
        let rules = OutdoorTimeEyeHealthRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("outdoor_eye"));
    }

    #[test]
    fn test_outdoortimeeyehealthrules_explain() {
        let rules = OutdoorTimeEyeHealthRules::new();
        let e = rules.explain();
        assert!(e.contains("户外价值"));
        assert!(e.contains("时长建议"));
        assert!(e.contains("活动类型"));
    }
}
