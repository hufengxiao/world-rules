//! 杠杆原理
//!
//! 杠杆的支点、力臂与省力原理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LeverPrincipleFulcrumRules,
    name: "杠杆原理",
    desc: "杠杆的支点、力臂与省力原理",
    origin: "物理",
    tags: ["科学", "杠杆", "物理", "原理"]
}

impl LeverPrincipleFulcrumRules {
    /// 杠杆要素
    pub fn parts(&self) -> Vec<&'static str> {
        vec!["支点力臂与重臂", "动力阻力两端", "围绕支点转动", "构成杠杆"]
    }

    /// 省力原理
    pub fn effort(&self) -> Vec<&'static str> {
        vec!["动力臂长省力", "支点近重物省力", "费力臂短费力", "距离换力"]
    }

    /// 三类杠杆
    pub fn classes(&self) -> Vec<&'static str> {
        vec!["支点中为一类", "阻力中为二类", "动力中为三类", "各有特性"]
    }

    /// 生活应用
    pub fn example(&self) -> Vec<&'static str> {
        vec!["撬棍开箱省力", "剪刀属杠杆", "天平秤平衡", "古来常用"]
    }
}

impl Rule for LeverPrincipleFulcrumRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("lever")
    }

    fn explain(&self) -> String {
        format!(
            "【杠杆原理】\n{}",
            [
                format!(
                    "杠杆要素：\\n{}",
                    self.parts()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "省力原理：\\n{}",
                    self.effort()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "三类杠杆：\\n{}",
                    self.classes()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活应用：\\n{}",
                    self.example()
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
    fn test_leverprinciplefulcrumrules_basic() {
        let rules = LeverPrincipleFulcrumRules::new();
        assert_eq!(rules.metadata().name, "杠杆原理");
        assert!(!rules.parts().is_empty());
        assert!(!rules.effort().is_empty());
        assert!(!rules.classes().is_empty());
        assert!(!rules.example().is_empty());
    }

    #[test]
    fn test_leverprinciplefulcrumrules_validation() {
        let rules = LeverPrincipleFulcrumRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("lever"));
    }

    #[test]
    fn test_leverprinciplefulcrumrules_explain() {
        let rules = LeverPrincipleFulcrumRules::new();
        let e = rules.explain();
        assert!(e.contains("杠杆要素"));
        assert!(e.contains("省力原理"));
        assert!(e.contains("三类杠杆"));
    }
}
