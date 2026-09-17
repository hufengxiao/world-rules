//! 营养素基础知识
//!
//! 宏量与微量营养素、食物选择的基础知识规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NutritionBasicsRules,
    name: "营养素基础知识",
    desc: "宏量与微量营养素、食物选择的基础知识规则",
    origin: "国际",
    tags: ["健康", "营养", "营养素", "膳食"]
}

impl NutritionBasicsRules {
    /// 宏量营养素
    pub fn macros(&self) -> Vec<&'static str> {
        vec![
            "碳水提供主要能量",
            "蛋白质支持修复与免疫",
            "脂肪帮助吸收功能",
            "合理搭配三者的比例",
        ]
    }

    /// 微量营养素
    pub fn micros(&self) -> Vec<&'static str> {
        vec![
            "关注维他命与矿物质",
            "摄取富含维C铁的蔬果",
            "钙与维D利于骨骼",
            "适度获取含锌含B族食物",
        ]
    }

    /// 读食品标签
    pub fn labels(&self) -> Vec<&'static str> {
        vec![
            "关注热量与营养素表",
            "留意配料排列顺序",
            "警惕高糖高钠隐藏值",
            "比较同类可选更优",
        ]
    }

    /// 均衡获取
    pub fn range(&self) -> Vec<&'static str> {
        vec![
            "从多样天然食物获取营养",
            "避免过度依赖补剂",
            "不盲目忌口偏食",
            "有疑虑征询专业人士",
        ]
    }
}

impl Rule for NutritionBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("nutrition_basics")
    }

    fn explain(&self) -> String {
        format!(
            "【营养素基础知识】\n{}",
            [
                format!(
                    "宏量营养素：\\n{}",
                    self.macros()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "微量营养素：\\n{}",
                    self.micros()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "读食品标签：\\n{}",
                    self.labels()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "均衡获取：\\n{}",
                    self.range()
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
    fn test_nutritionbasicsrules_basic() {
        let rules = NutritionBasicsRules::new();
        assert_eq!(rules.metadata().name, "营养素基础知识");
        assert!(!rules.macros().is_empty());
        assert!(!rules.micros().is_empty());
        assert!(!rules.labels().is_empty());
        assert!(!rules.range().is_empty());
    }

    #[test]
    fn test_nutritionbasicsrules_validation() {
        let rules = NutritionBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("nutrition_basics"));
    }

    #[test]
    fn test_nutritionbasicsrules_explain() {
        let rules = NutritionBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("宏量营养素"));
        assert!(e.contains("微量营养素"));
        assert!(e.contains("读食品标签"));
    }
}
