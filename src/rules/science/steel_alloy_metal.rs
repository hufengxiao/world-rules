//! 钢与合金
//!
//! 钢、合金的性能优势与常见金属

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SteelAlloyMetalRules,
    name: "钢与合金",
    desc: "钢、合金的性能优势与常见金属",
    origin: "化学",
    tags: ["科学", "钢", "合金", "金属"]
}

impl SteelAlloyMetalRules {
    /// 钢的组成
    pub fn steel(&self) -> Vec<&'static str> {
        vec!["钢是铁碳合金", "含碳少量", "强度硬度高", "广泛用于建筑"]
    }

    /// 合金优势
    pub fn alloy(&self) -> Vec<&'static str> {
        vec!["合金强度高", "耐蚀抗磨", "可塑可控", "性能优良"]
    }

    /// 常见合金
    pub fn examples(&self) -> Vec<&'static str> {
        vec!["不锈钢耐锈", "铝合金轻", "青铜为铜锡合金", "应用广泛"]
    }

    /// 选用合理
    pub fn usage(&self) -> Vec<&'static str> {
        vec!["按需选材", "金属回收", "合理开采", "珍惜资源"]
    }
}

impl Rule for SteelAlloyMetalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("steel_alloy")
    }

    fn explain(&self) -> String {
        format!(
            "【钢与合金】\n{}",
            [
                format!(
                    "钢的组成：\\n{}",
                    self.steel()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "合金优势：\\n{}",
                    self.alloy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "常见合金：\\n{}",
                    self.examples()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "选用合理：\\n{}",
                    self.usage()
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
    fn test_steelalloymetalrules_basic() {
        let rules = SteelAlloyMetalRules::new();
        assert_eq!(rules.metadata().name, "钢与合金");
        assert!(!rules.steel().is_empty());
        assert!(!rules.alloy().is_empty());
        assert!(!rules.examples().is_empty());
        assert!(!rules.usage().is_empty());
    }

    #[test]
    fn test_steelalloymetalrules_validation() {
        let rules = SteelAlloyMetalRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("steel_alloy"));
    }

    #[test]
    fn test_steelalloymetalrules_explain() {
        let rules = SteelAlloyMetalRules::new();
        let e = rules.explain();
        assert!(e.contains("钢的组成"));
        assert!(e.contains("合金优势"));
        assert!(e.contains("常见合金"));
    }
}
