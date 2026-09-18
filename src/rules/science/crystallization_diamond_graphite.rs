//! 碳结晶与金刚石石墨
//!
//! 金刚石石墨同素异形体的构成差异

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CrystallizationDiamondGraphiteRules,
    name: "碳结晶与金刚石石墨",
    desc: "金刚石石墨同素异形体的构成差异",
    origin: "化学",
    tags: ["科学", "碳", "金刚石", "石墨"]
}

impl CrystallizationDiamondGraphiteRules {
    /// 同为碳元素
    pub fn same(&self) -> Vec<&'static str> {
        vec![
            "金刚石墨都由碳组成",
            "是碳的同素异形体",
            "结构决定性质",
            "性质迥异",
        ]
    }

    /// 金刚石
    pub fn diamond(&self) -> Vec<&'static str> {
        vec!["碳原子紧密成网", "结构极硬", "用于切割磨具", "坚硬耐磨"]
    }

    /// 石墨
    pub fn graphite(&self) -> Vec<&'static str> {
        vec!["层状结构易滑", "导电导热软", "铅笔芯用石墨", "润滑可用"]
    }

    /// 人工转化
    pub fn synthesis(&self) -> Vec<&'static str> {
        vec!["高温高压可转化", "性能各异", "科学应用", "物性可究"]
    }
}

impl Rule for CrystallizationDiamondGraphiteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("carbon_crystal")
    }

    fn explain(&self) -> String {
        format!(
            "【碳结晶与金刚石石墨】\n{}",
            [
                format!(
                    "同为碳元素：\\n{}",
                    self.same()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "金刚石：\\n{}",
                    self.diamond()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "石墨：\\n{}",
                    self.graphite()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "人工转化：\\n{}",
                    self.synthesis()
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
    fn test_crystallizationdiamondgraphiterules_basic() {
        let rules = CrystallizationDiamondGraphiteRules::new();
        assert_eq!(rules.metadata().name, "碳结晶与金刚石石墨");
        assert!(!rules.same().is_empty());
        assert!(!rules.diamond().is_empty());
        assert!(!rules.graphite().is_empty());
        assert!(!rules.synthesis().is_empty());
    }

    #[test]
    fn test_crystallizationdiamondgraphiterules_validation() {
        let rules = CrystallizationDiamondGraphiteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("carbon_crystal"));
    }

    #[test]
    fn test_crystallizationdiamondgraphiterules_explain() {
        let rules = CrystallizationDiamondGraphiteRules::new();
        let e = rules.explain();
        assert!(e.contains("同为碳元素"));
        assert!(e.contains("金刚石"));
        assert!(e.contains("石墨"));
    }
}
