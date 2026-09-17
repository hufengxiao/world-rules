//! 老人安全用药
//!
//! 老人按时服药、分装与用药安全

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ElderMedicationRemindRules,
    name: "老人安全用药",
    desc: "老人按时服药、分装与用药安全",
    origin: "医学",
    tags: ["健康", "老人", "用药", "安全"]
}

impl ElderMedicationRemindRules {
    /// 按时服药
    pub fn timing(&self) -> Vec<&'static str> {
        vec!["按医嘱定时吃", "不随意停药", "药盒提醒", "漏服按规补"]
    }

    /// 分装记录
    pub fn organize(&self) -> Vec<&'static str> {
        vec!["分类分格药盒", "标注日期药名", "记录服药单", "家人可代管"]
    }

    /// 避免误用
    pub fn avoid(&self) -> Vec<&'static str> {
        vec!["不过量不混吃", "不擅自加药", "过期药弃用", "药物相忌问医"]
    }

    /// 反应观察
    pub fn observe(&self) -> Vec<&'static str> {
        vec!["留意不适反应", "头晕嗜睡注意", "异常就症", "安全检查"]
    }
}

impl Rule for ElderMedicationRemindRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("elder_medication")
    }

    fn explain(&self) -> String {
        format!(
            "【老人安全用药】\n{}",
            [
                format!(
                    "按时服药：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "分装记录：\\n{}",
                    self.organize()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "避免误用：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "反应观察：\\n{}",
                    self.observe()
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
    fn test_eldermedicationremindrules_basic() {
        let rules = ElderMedicationRemindRules::new();
        assert_eq!(rules.metadata().name, "老人安全用药");
        assert!(!rules.timing().is_empty());
        assert!(!rules.organize().is_empty());
        assert!(!rules.avoid().is_empty());
        assert!(!rules.observe().is_empty());
    }

    #[test]
    fn test_eldermedicationremindrules_validation() {
        let rules = ElderMedicationRemindRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("elder_medication"));
    }

    #[test]
    fn test_eldermedicationremindrules_explain() {
        let rules = ElderMedicationRemindRules::new();
        let e = rules.explain();
        assert!(e.contains("按时服药"));
        assert!(e.contains("分装记录"));
        assert!(e.contains("避免误用"));
    }
}
