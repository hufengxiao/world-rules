//! 孕期营养
//!
//! 孕期各阶段膳食营养与补充要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PregnancyNutritionRules,
    name: "孕期营养",
    desc: "孕期各阶段膳食营养与补充要点",
    origin: "中国",
    tags: ["孕产", "营养", "女性"]
}

impl PregnancyNutritionRules {
    /// 关键营养
    pub fn nutrients(&self) -> Vec<&'static str> {
        vec![
            "叶酸预防神经管缺陷",
            "足量优质蛋白摄入",
            "补铁预防孕期贫血",
            "钙与维生素D兼顾",
        ]
    }

    /// 分阶段膳食
    pub fn stages(&self) -> Vec<&'static str> {
        vec![
            "孕早期清淡少食多餐",
            "孕中期增加蛋白质",
            "孕晚期控制总量防过大",
            "全期均衡不挑食",
        ]
    }

    /// 饮食禁忌
    pub fn avoid(&self) -> Vec<&'static str> {
        vec![
            "忌生食与未熟肉类",
            "慎食高汞鱼类",
            "限制咖啡因摄入",
            "戒烟戒酒最为关键",
        ]
    }

    /// 体重管理
    pub fn weight(&self) -> Vec<&'static str> {
        vec![
            "按孕前体重定增重目标",
            "定期监测体重变化",
            "增重过快须调整饮食",
            "适度活动助控体重",
        ]
    }
}

impl Rule for PregnancyNutritionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("pregnancy_nutrition")
    }

    fn explain(&self) -> String {
        format!(
            "【孕期营养】\n{}",
            [
                format!(
                    "关键营养：\\n{}",
                    self.nutrients()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "分阶段膳食：\\n{}",
                    self.stages()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食禁忌：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "体重管理：\\n{}",
                    self.weight()
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
    fn test_pregnancynutritionrules_basic() {
        let rules = PregnancyNutritionRules::new();
        assert_eq!(rules.metadata().name, "孕期营养");
        assert!(!rules.nutrients().is_empty());
        assert!(!rules.stages().is_empty());
        assert!(!rules.avoid().is_empty());
        assert!(!rules.weight().is_empty());
    }

    #[test]
    fn test_pregnancynutritionrules_validation() {
        let rules = PregnancyNutritionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("pregnancy_nutrition")
        );
    }

    #[test]
    fn test_pregnancynutritionrules_explain() {
        let rules = PregnancyNutritionRules::new();
        let e = rules.explain();
        assert!(e.contains("关键营养"));
        assert!(e.contains("分阶段膳食"));
        assert!(e.contains("饮食禁忌"));
    }
}
