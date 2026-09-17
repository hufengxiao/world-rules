//! 均衡餐盘搭配
//!
//! 每餐主食、蔬菜、蛋白的比例搭配

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BalancedDietPlateRules,
    name: "均衡餐盘搭配",
    desc: "每餐主食、蔬菜、蛋白的比例搭配",
    origin: "营养学",
    tags: ["健康", "饮食", "均衡", "餐盘"]
}

impl BalancedDietPlateRules {
    /// 餐盘比例
    pub fn ratio(&self) -> Vec<&'static str> {
        vec![
            "半盘蔬菜水果",
            "四分之一主食",
            "四分之一蛋白质",
            "少量健康油",
        ]
    }

    /// 主食多样
    pub fn staple(&self) -> Vec<&'static str> {
        vec!["多选全谷物", "粗细搭配", "少精糖白面", "适量薯类"]
    }

    /// 蔬菜为主
    pub fn vegetable(&self) -> Vec<&'static str> {
        vec!["多种颜色蔬菜", "每餐有绿叶", "果蔬充足", "种类多样"]
    }

    /// 蛋白适量
    pub fn protein(&self) -> Vec<&'static str> {
        vec!["鱼禽蛋奶适量", "豆类与瘦肉", "不偏不废", "量按体重"]
    }
}

impl Rule for BalancedDietPlateRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("diet_plate")
    }

    fn explain(&self) -> String {
        format!(
            "【均衡餐盘搭配】\n{}",
            [
                format!(
                    "餐盘比例：\\n{}",
                    self.ratio()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "主食多样：\\n{}",
                    self.staple()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "蔬菜为主：\\n{}",
                    self.vegetable()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "蛋白适量：\\n{}",
                    self.protein()
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
    fn test_balanceddietplaterules_basic() {
        let rules = BalancedDietPlateRules::new();
        assert_eq!(rules.metadata().name, "均衡餐盘搭配");
        assert!(!rules.ratio().is_empty());
        assert!(!rules.staple().is_empty());
        assert!(!rules.vegetable().is_empty());
        assert!(!rules.protein().is_empty());
    }

    #[test]
    fn test_balanceddietplaterules_validation() {
        let rules = BalancedDietPlateRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("diet_plate"));
    }

    #[test]
    fn test_balanceddietplaterules_explain() {
        let rules = BalancedDietPlateRules::new();
        let e = rules.explain();
        assert!(e.contains("餐盘比例"));
        assert!(e.contains("主食多样"));
        assert!(e.contains("蔬菜为主"));
    }
}
