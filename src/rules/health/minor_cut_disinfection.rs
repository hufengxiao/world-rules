//! 小伤口消毒处理
//!
//! 小擦伤割伤的清洁消毒与包扎

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MinorCutDisinfectionRules,
    name: "小伤口消毒处理",
    desc: "小擦伤割伤的清洁消毒与包扎",
    origin: "医学",
    tags: ["健康", "伤口", "消毒", "包扎"]
}

impl MinorCutDisinfectionRules {
    /// 清洁止血
    pub fn clean(&self) -> Vec<&'static str> {
        vec!["先洗净双手", "冲洗伤口污物", "无菌敷料按压", "止住出血"]
    }

    /// 消毒处理
    pub fn disinfect(&self) -> Vec<&'static str> {
        vec!["碘伏消毒", "由内往外擦", "酒精清洁", "不覆盖湿物"]
    }

    /// 包扎防感
    pub fn bandage(&self) -> Vec<&'static str> {
        vec!["创口贴包", "透气纱布", "定期换药", "保持干燥"]
    }

    /// 异常就医
    pub fn visit(&self) -> Vec<&'static str> {
        vec!["出血不止就医", "深伤破伤风", "化脓红肿注意", "早处理"]
    }
}

impl Rule for MinorCutDisinfectionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("minor_cut")
    }

    fn explain(&self) -> String {
        format!(
            "【小伤口消毒处理】\n{}",
            [
                format!(
                    "清洁止血：\\n{}",
                    self.clean()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "消毒处理：\\n{}",
                    self.disinfect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "包扎防感：\\n{}",
                    self.bandage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "异常就医：\\n{}",
                    self.visit()
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
    fn test_minorcutdisinfectionrules_basic() {
        let rules = MinorCutDisinfectionRules::new();
        assert_eq!(rules.metadata().name, "小伤口消毒处理");
        assert!(!rules.clean().is_empty());
        assert!(!rules.disinfect().is_empty());
        assert!(!rules.bandage().is_empty());
        assert!(!rules.visit().is_empty());
    }

    #[test]
    fn test_minorcutdisinfectionrules_validation() {
        let rules = MinorCutDisinfectionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("minor_cut"));
    }

    #[test]
    fn test_minorcutdisinfectionrules_explain() {
        let rules = MinorCutDisinfectionRules::new();
        let e = rules.explain();
        assert!(e.contains("清洁止血"));
        assert!(e.contains("消毒处理"));
        assert!(e.contains("包扎防感"));
    }
}
