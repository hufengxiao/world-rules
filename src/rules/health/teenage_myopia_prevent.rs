//! 青少年近视预防
//!
//! 青少年用眼卫生、户外预防近视

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TeenageMyopiaPreventRules,
    name: "青少年近视预防",
    desc: "青少年用眼卫生、户外预防近视",
    origin: "医学",
    tags: ["健康", "近视", "青少年", "护眼"]
}

impl TeenageMyopiaPreventRules {
    /// 用眼距离
    pub fn distance(&self) -> Vec<&'static str> {
        vec!["书桌保持一尺", "屏幕适度远", "端正坐姿", "保持距离适中"]
    }

    /// 定时休息
    pub fn rest(&self) -> Vec<&'static str> {
        vec!["用心每二十分钟", "远眺放松", "少低头", "遵休息规律"]
    }

    /// 户外活动
    pub fn outdoor(&self) -> Vec<&'static str> {
        vec!["每日户外两小时", "多晒太阳目视远", "促进视力", "防近视主力"]
    }

    /// 定期检查
    pub fn check(&self) -> Vec<&'static str> {
        vec!["定期测视力", "发现近视及时配", "遵医嘱", "科学干预"]
    }
}

impl Rule for TeenageMyopiaPreventRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("myopia")
    }

    fn explain(&self) -> String {
        format!(
            "【青少年近视预防】\n{}",
            [
                format!(
                    "用眼距离：\\n{}",
                    self.distance()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "定时休息：\\n{}",
                    self.rest()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "户外活动：\\n{}",
                    self.outdoor()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "定期检查：\\n{}",
                    self.check()
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
    fn test_teenagemyopiapreventrules_basic() {
        let rules = TeenageMyopiaPreventRules::new();
        assert_eq!(rules.metadata().name, "青少年近视预防");
        assert!(!rules.distance().is_empty());
        assert!(!rules.rest().is_empty());
        assert!(!rules.outdoor().is_empty());
        assert!(!rules.check().is_empty());
    }

    #[test]
    fn test_teenagemyopiapreventrules_validation() {
        let rules = TeenageMyopiaPreventRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("myopia"));
    }

    #[test]
    fn test_teenagemyopiapreventrules_explain() {
        let rules = TeenageMyopiaPreventRules::new();
        let e = rules.explain();
        assert!(e.contains("用眼距离"));
        assert!(e.contains("定时休息"));
        assert!(e.contains("户外活动"));
    }
}
