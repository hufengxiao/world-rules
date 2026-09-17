//! 烹饪化学常识
//!
//! 烹饪中的美拉德反应、酸碱与食物化学常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CookingChemistryRules,
    name: "烹饪化学常识",
    desc: "烹饪中的美拉德反应、酸碱与食物化学常识",
    origin: "国际",
    tags: ["科学", "烹饪", "化学", "食品"]
}

impl CookingChemistryRules {
    /// 加热变化
    pub fn heat(&self) -> Vec<&'static str> {
        vec![
            "美拉德反应产生色泽风味",
            "蛋白质高温变性凝固",
            "淀粉糊化黏稠",
            "油脂加热氧化变苦",
        ]
    }

    /// 酸碱应用
    pub fn acid(&self) -> Vec<&'static str> {
        vec![
            "酸能嫩化食材",
            "碱可软化纤维",
            "发酵靠微生物产酸",
            "pH影响口感颜色",
        ]
    }

    /// 乳化与勾芡
    pub fn emulsion(&self) -> Vec<&'static str> {
        vec![
            "蛋黄乳化酱汁",
            "油脂与水需乳化剂",
            "勾芡让汤汁浓稠",
            "不同油温影响口感",
        ]
    }

    /// 科学习惯
    pub fn habit(&self) -> Vec<&'static str> {
        vec![
            "掌握火候与调配",
            "健康少油盐",
            "不烹调过度损失营养",
            "科学处理剩菜时限",
        ]
    }
}

impl Rule for CookingChemistryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("cooking_chemistry")
    }

    fn explain(&self) -> String {
        format!(
            "【烹饪化学常识】\n{}",
            [
                format!(
                    "加热变化：\\n{}",
                    self.heat()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "酸碱应用：\\n{}",
                    self.acid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "乳化与勾芡：\\n{}",
                    self.emulsion()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "科学习惯：\\n{}",
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
    fn test_cookingchemistryrules_basic() {
        let rules = CookingChemistryRules::new();
        assert_eq!(rules.metadata().name, "烹饪化学常识");
        assert!(!rules.heat().is_empty());
        assert!(!rules.acid().is_empty());
        assert!(!rules.emulsion().is_empty());
        assert!(!rules.habit().is_empty());
    }

    #[test]
    fn test_cookingchemistryrules_validation() {
        let rules = CookingChemistryRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("cooking_chemistry"));
    }

    #[test]
    fn test_cookingchemistryrules_explain() {
        let rules = CookingChemistryRules::new();
        let e = rules.explain();
        assert!(e.contains("加热变化"));
        assert!(e.contains("酸碱应用"));
        assert!(e.contains("乳化与勾芡"));
    }
}
