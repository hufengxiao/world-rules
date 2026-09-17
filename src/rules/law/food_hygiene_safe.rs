//! 食品安全维权
//!
//! 食品变质、卫生问题的投诉与赔偿

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FoodHygieneSafeRules,
    name: "食品安全维权",
    desc: "食品变质、卫生问题的投诉与赔偿",
    origin: "中国",
    tags: ["消费", "食品", "安全", "维权"]
}

impl FoodHygieneSafeRules {
    /// 发现问题
    pub fn identify(&self) -> Vec<&'static str> {
        vec!["留意异味异物", "过期变质不食", "保存问题食品", "留取样品"]
    }

    /// 即时处理
    pub fn respond(&self) -> Vec<&'static str> {
        vec![
            "停止食用不适就医",
            "暂留票据",
            "必要时报市场监管",
            "保留记录",
        ]
    }

    /// 合理索赔
    pub fn claim(&self) -> Vec<&'static str> {
        vec!["向商家反映", "依法要求赔偿", "损失依证计算", "不过分索赔"]
    }

    /// 举报维权
    pub fn report(&self) -> Vec<&'static str> {
        vec!["向监管部门举报", "拨打热线", "依实举报", "维护安全"]
    }
}

impl Rule for FoodHygieneSafeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("food_hygiene")
    }

    fn explain(&self) -> String {
        format!(
            "【食品安全维权】\n{}",
            [
                format!(
                    "发现问题：\\n{}",
                    self.identify()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "即时处理：\\n{}",
                    self.respond()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "合理索赔：\\n{}",
                    self.claim()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "举报维权：\\n{}",
                    self.report()
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
    fn test_foodhygienesaferules_basic() {
        let rules = FoodHygieneSafeRules::new();
        assert_eq!(rules.metadata().name, "食品安全维权");
        assert!(!rules.identify().is_empty());
        assert!(!rules.respond().is_empty());
        assert!(!rules.claim().is_empty());
        assert!(!rules.report().is_empty());
    }

    #[test]
    fn test_foodhygienesaferules_validation() {
        let rules = FoodHygieneSafeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("food_hygiene"));
    }

    #[test]
    fn test_foodhygienesaferules_explain() {
        let rules = FoodHygieneSafeRules::new();
        let e = rules.explain();
        assert!(e.contains("发现问题"));
        assert!(e.contains("即时处理"));
        assert!(e.contains("合理索赔"));
    }
}
