//! 氧化还原反应
//!
//! 电子转移引发的氧化还原反应基本原理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OxidationReductionRules,
    name: "氧化还原反应",
    desc: "电子转移引发的氧化还原反应基本原理",
    origin: "科学常识",
    tags: ["化学", "反应", "电子"]
}

impl OxidationReductionRules {
    /// 基本概念
    pub fn concept(&self) -> Vec<&'static str> {
        vec![
            "失电子为氧化",
            "得电子为还原",
            "两者必然同时发生",
            "电子守恒是核心",
        ]
    }

    /// 化合价变化
    pub fn valence(&self) -> Vec<&'static str> {
        vec![
            "化合价升高被氧化",
            "化合价降低被还原",
            "升降总数相等",
            "据此配平方程式",
        ]
    }

    /// 氧化剂还原剂
    pub fn agents(&self) -> Vec<&'static str> {
        vec![
            "氧化剂得电子被还原",
            "还原剂失电子被氧化",
            "氧化剂具氧化性",
            "强弱由得失电子难易决定",
        ]
    }

    /// 生活实例
    pub fn examples(&self) -> Vec<&'static str> {
        vec![
            "铁生锈是氧化",
            "燃烧属剧烈氧化",
            "电池利用氧化还原",
            "漂白消毒涉氧化还原",
        ]
    }
}

impl Rule for OxidationReductionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("oxidation_reduction")
    }

    fn explain(&self) -> String {
        format!(
            "【氧化还原反应】\n{}",
            [
                format!(
                    "基本概念：\\n{}",
                    self.concept()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "化合价变化：\\n{}",
                    self.valence()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "氧化剂还原剂：\\n{}",
                    self.agents()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活实例：\\n{}",
                    self.examples()
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
    fn test_oxidationreductionrules_basic() {
        let rules = OxidationReductionRules::new();
        assert_eq!(rules.metadata().name, "氧化还原反应");
        assert!(!rules.concept().is_empty());
        assert!(!rules.valence().is_empty());
        assert!(!rules.agents().is_empty());
        assert!(!rules.examples().is_empty());
    }

    #[test]
    fn test_oxidationreductionrules_validation() {
        let rules = OxidationReductionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("oxidation_reduction")
        );
    }

    #[test]
    fn test_oxidationreductionrules_explain() {
        let rules = OxidationReductionRules::new();
        let e = rules.explain();
        assert!(e.contains("基本概念"));
        assert!(e.contains("化合价变化"));
        assert!(e.contains("氧化剂还原剂"));
    }
}
