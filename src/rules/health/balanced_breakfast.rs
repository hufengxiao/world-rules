//! 均衡早餐搭配
//!
//! 早餐主食、蛋白、蔬果搭配与肠胃呵护

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BalancedBreakfastRules,
    name: "均衡早餐搭配",
    desc: "早餐主食、蛋白、蔬果搭配与肠胃呵护",
    origin: "营养学",
    tags: ["健康", "早餐", "均衡", "营养"]
}

impl BalancedBreakfastRules {
    /// 营养搭配
    pub fn mix(&self) -> Vec<&'static str> {
        vec!["含主食全谷", "配蛋白质蛋奶", "加点蔬果", "少油腻少糖"]
    }

    /// 早餐原件
    pub fn component(&self) -> Vec<&'static str> {
        vec!["全麦面包或粥", "鸡蛋牛奶酸奶", "水果或凉菜", "营养全面"]
    }

    /// 按时吃
    pub fn timing(&self) -> Vec<&'static str> {
        vec!["起床后适时吃", "不空腹太久", "早餐不宜跳过", "细嚼慢咽"]
    }

    /// 长幼兼顾
    pub fn habit(&self) -> Vec<&'static str> {
        vec![
            "家人一起用餐",
            "为儿童保证营养",
            "老人软食易消化",
            "培养健康晨序",
        ]
    }
}

impl Rule for BalancedBreakfastRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("breakfast")
    }

    fn explain(&self) -> String {
        format!(
            "【均衡早餐搭配】\n{}",
            [
                format!(
                    "营养搭配：\\n{}",
                    self.mix()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "早餐原件：\\n{}",
                    self.component()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "按时吃：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "长幼兼顾：\\n{}",
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
    fn test_balancedbreakfastrules_basic() {
        let rules = BalancedBreakfastRules::new();
        assert_eq!(rules.metadata().name, "均衡早餐搭配");
        assert!(!rules.mix().is_empty());
        assert!(!rules.component().is_empty());
        assert!(!rules.timing().is_empty());
        assert!(!rules.habit().is_empty());
    }

    #[test]
    fn test_balancedbreakfastrules_validation() {
        let rules = BalancedBreakfastRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("breakfast"));
    }

    #[test]
    fn test_balancedbreakfastrules_explain() {
        let rules = BalancedBreakfastRules::new();
        let e = rules.explain();
        assert!(e.contains("营养搭配"));
        assert!(e.contains("早餐原件"));
        assert!(e.contains("按时吃"));
    }
}
