//! 西餐礼仪
//!
//! 西餐刀叉使用、用餐顺序与会谈礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WesternMealEtiquetteRules,
    name: "西餐礼仪",
    desc: "西餐刀叉使用、用餐顺序与会谈礼节",
    origin: "国际",
    tags: ["社交", "西餐", "礼仪", "餐桌"]
}

impl WesternMealEtiquetteRules {
    /// 刀叉用法
    pub fn utensils(&self) -> Vec<&'static str> {
        vec!["左叉右刀", "从外到内取用", "切一小块吃一口", "不叉举过高"]
    }

    /// 用餐顺序
    pub fn courses(&self) -> Vec<&'static str> {
        vec!["前菜汤到主菜", "甜点咖啡收尾", "由外向内依次", "按序享用"]
    }

    /// 面包汤类
    pub fn details(&self) -> Vec<&'static str> {
        vec!["面包掰小块吃", "勺汤向外舀", "不吹气凉汤", "轻持杯"]
    }

    /// 礼貌交谈
    pub fn converse(&self) -> Vec<&'static str> {
        vec!["咀嚼不张口说", "音量适中", "等主客说话", "尊重同桌"]
    }
}

impl Rule for WesternMealEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("western_meal")
    }

    fn explain(&self) -> String {
        format!(
            "【西餐礼仪】\n{}",
            [
                format!(
                    "刀叉用法：\\n{}",
                    self.utensils()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用餐顺序：\\n{}",
                    self.courses()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "面包汤类：\\n{}",
                    self.details()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "礼貌交谈：\\n{}",
                    self.converse()
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
    fn test_westernmealetiquetterules_basic() {
        let rules = WesternMealEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "西餐礼仪");
        assert!(!rules.utensils().is_empty());
        assert!(!rules.courses().is_empty());
        assert!(!rules.details().is_empty());
        assert!(!rules.converse().is_empty());
    }

    #[test]
    fn test_westernmealetiquetterules_validation() {
        let rules = WesternMealEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("western_meal"));
    }

    #[test]
    fn test_westernmealetiquetterules_explain() {
        let rules = WesternMealEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("刀叉用法"));
        assert!(e.contains("用餐顺序"));
        assert!(e.contains("面包汤类"));
    }
}
