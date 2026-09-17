//! 膳食纤维膳食
//!
//! 膳食纤维作用、来源与足量蔬菜摄入

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DietaryFiberVegetableRules,
    name: "膳食纤维膳食",
    desc: "膳食纤维作用、来源与足量蔬菜摄入",
    origin: "营养学",
    tags: ["健康", "膳食纤维", "蔬菜", "肠道"]
}

impl DietaryFiberVegetableRules {
    /// 纤维作用
    pub fn benefit(&self) -> Vec<&'static str> {
        vec!["促进肠道蠕动", "稳定血糖血脂", "增加饱腹感", "维护肠道健康"]
    }

    /// 足量蔬菜
    pub fn vegetable(&self) -> Vec<&'static str> {
        vec!["每天多种蔬菜", "少油清炒保纤维", "深色菜多吃", "不熬制过烂"]
    }

    /// 谷薯坚果
    pub fn extra(&self) -> Vec<&'static str> {
        vec!["全谷物含纤维", "薯类豆类要多", "坚果适量", "粗粮搭配"]
    }

    /// 循序渐进
    pub fn gradual(&self) -> Vec<&'static str> {
        vec!["逐步增加纤维", "配合足够饮水", "肠胃不适调整", "均衡不偏废"]
    }
}

impl Rule for DietaryFiberVegetableRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("fiber")
    }

    fn explain(&self) -> String {
        format!(
            "【膳食纤维膳食】\n{}",
            [
                format!(
                    "纤维作用：\\n{}",
                    self.benefit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "足量蔬菜：\\n{}",
                    self.vegetable()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "谷薯坚果：\\n{}",
                    self.extra()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "循序渐进：\\n{}",
                    self.gradual()
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
    fn test_dietaryfibervegetablerules_basic() {
        let rules = DietaryFiberVegetableRules::new();
        assert_eq!(rules.metadata().name, "膳食纤维膳食");
        assert!(!rules.benefit().is_empty());
        assert!(!rules.vegetable().is_empty());
        assert!(!rules.extra().is_empty());
        assert!(!rules.gradual().is_empty());
    }

    #[test]
    fn test_dietaryfibervegetablerules_validation() {
        let rules = DietaryFiberVegetableRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("fiber"));
    }

    #[test]
    fn test_dietaryfibervegetablerules_explain() {
        let rules = DietaryFiberVegetableRules::new();
        let e = rules.explain();
        assert!(e.contains("纤维作用"));
        assert!(e.contains("足量蔬菜"));
        assert!(e.contains("谷薯坚果"));
    }
}
