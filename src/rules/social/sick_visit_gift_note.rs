//! 探病送礼
//!
//! 探病带什么礼、禁忌与合适表示

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SickVisitGiftNoteRules,
    name: "探病送礼",
    desc: "探病带什么礼、禁忌与合适表示",
    origin: "中国",
    tags: ["社交", "探病", "礼品", "禁忌"]
}

impl SickVisitGiftNoteRules {
    /// 合适礼物
    pub fn suitable(&self) -> Vec<&'static str> {
        vec!["带水果营养品", "鲜花最合宜", "实用日用品", "表达关怀"]
    }

    /// 禁忌注意
    pub fn taboo(&self) -> Vec<&'static str> {
        vec!["先问忌口", "避开不宜食物", "不送钟表等", "颜色忌讳"]
    }

    /// 病况问清
    pub fn ask(&self) -> Vec<&'static str> {
        vec!["了解能否进食", "糖尿病慎甜", "手术前免送食", "贴己心意"]
    }

    /// 心意为主
    pub fn heart(&self) -> Vec<&'static str> {
        vec!["礼物不必贵重", "关怀胜于物", "实用体贴", "祝早日康复"]
    }
}

impl Rule for SickVisitGiftNoteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("sick_gift")
    }

    fn explain(&self) -> String {
        format!(
            "【探病送礼】\n{}",
            [
                format!(
                    "合适礼物：\\n{}",
                    self.suitable()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "禁忌注意：\\n{}",
                    self.taboo()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "病况问清：\\n{}",
                    self.ask()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "心意为主：\\n{}",
                    self.heart()
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
    fn test_sickvisitgiftnoterules_basic() {
        let rules = SickVisitGiftNoteRules::new();
        assert_eq!(rules.metadata().name, "探病送礼");
        assert!(!rules.suitable().is_empty());
        assert!(!rules.taboo().is_empty());
        assert!(!rules.ask().is_empty());
        assert!(!rules.heart().is_empty());
    }

    #[test]
    fn test_sickvisitgiftnoterules_validation() {
        let rules = SickVisitGiftNoteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("sick_gift"));
    }

    #[test]
    fn test_sickvisitgiftnoterules_explain() {
        let rules = SickVisitGiftNoteRules::new();
        let e = rules.explain();
        assert!(e.contains("合适礼物"));
        assert!(e.contains("禁忌注意"));
        assert!(e.contains("病况问清"));
    }
}
