//! 素食均衡营养
//!
//! 纯素或其他素食者的营养搭配与补充原则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: VegetarianBalancedNutritionRules,
    name: "素食均衡营养",
    desc: "纯素或其他素食者的营养搭配与补充原则",
    origin: "国际",
    tags: ["健康", "素食", "营养", "蛋白", "规律"]
}

impl VegetarianBalancedNutritionRules {
    /// 蛋白质来源
    pub fn protein(&self) -> Vec<&'static str> {
        vec![
            "摄取豆类坚果豆腐等植物蛋白",
            "兼顾不同来源提供完整氨基酸",
            "注意份量充足",
            "混合谷物提高蛋白质量",
        ]
    }

    /// 关键营养
    pub fn nutrients(&self) -> Vec<&'static str> {
        vec![
            "关注维生素B12来源",
            "素食者注意铁与锌补充",
            "关注钙与维D",
            "可能需要适量相关补剂",
        ]
    }

    /// 饮食多样
    pub fn variety(&self) -> Vec<&'static str> {
        vec![
            "多样化植物性食物",
            "蔬果全谷豆类坚果搭配",
            "注意脂肪质量不过量",
            "保留足够能量摄入",
        ]
    }

    /// 个体咨询
    pub fn guidance(&self) -> Vec<&'static str> {
        vec![
            "儿童孕产妇素食需专业指导",
            "特殊情况下就医评估",
            "不盲信极端素食",
            "据身体反馈调整",
        ]
    }
}

impl Rule for VegetarianBalancedNutritionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("vegetarian_nutrition")
    }

    fn explain(&self) -> String {
        format!(
            "【素食均衡营养】\n{}",
            [
                format!(
                    "蛋白质来源：\\n{}",
                    self.protein()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "关键营养：\\n{}",
                    self.nutrients()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食多样：\\n{}",
                    self.variety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "个体咨询：\\n{}",
                    self.guidance()
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
    fn test_vegetarianbalancednutritionrules_basic() {
        let rules = VegetarianBalancedNutritionRules::new();
        assert_eq!(rules.metadata().name, "素食均衡营养");
        assert!(!rules.protein().is_empty());
        assert!(!rules.nutrients().is_empty());
        assert!(!rules.variety().is_empty());
        assert!(!rules.guidance().is_empty());
    }

    #[test]
    fn test_vegetarianbalancednutritionrules_validation() {
        let rules = VegetarianBalancedNutritionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("vegetarian_nutrition")
        );
    }

    #[test]
    fn test_vegetarianbalancednutritionrules_explain() {
        let rules = VegetarianBalancedNutritionRules::new();
        let e = rules.explain();
        assert!(e.contains("蛋白质来源"));
        assert!(e.contains("关键营养"));
        assert!(e.contains("饮食多样"));
    }
}
