//! 家庭食品安全
//!
//! 家庭厨房中预防食源性疾病的选购、储存与烹调规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FoodSafetyRules,
    name: "家庭食品安全",
    desc: "家庭厨房中预防食源性疾病的选购、储存与烹调规则",
    origin: "医学",
    tags: ["健康", "食品", "安全", "卫生", "烹饪"]
}

impl FoodSafetyRules {
    /// 选购
    pub fn shopping(&self) -> Vec<&'static str> {
        vec![
            "选购新鲜且在保质期内食品",
            "购买冷藏冷冻食品注意冷链",
            "查看包装完整与生产日期",
            "不买涨袋罐装与异味蛋",
        ]
    }

    /// 储存
    pub fn storage(&self) -> Vec<&'static str> {
        vec![
            "生熟分开存放避免交叉",
            "冷藏温度低于4℃",
            "冷冻解冻按冷藏解冻方式",
            "剩菜及时放凉并当天食用",
        ]
    }

    /// 烹调
    pub fn cooking(&self) -> Vec<&'static str> {
        vec![
            "肉类禽蛋彻底煮熟",
            "中心温度达到充分杀菌",
            "蔬菜清洗处理",
            "刀具砧板生熟分开",
        ]
    }

    /// 卫生
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec![
            "饭前便后洗手",
            "处理生肉后清洁台面",
            "不食用变色变味剩菜",
            "及时清理厨余垃圾",
        ]
    }
}

impl Rule for FoodSafetyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("food_safety")
    }

    fn explain(&self) -> String {
        let parts = vec![
            format!(
                "选购：\\n{}",
                self.shopping()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "储存：\\n{}",
                self.storage()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "烹调：\\n{}",
                self.cooking()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "卫生：\\n{}",
                self.hygiene()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
        ];
        format!("【家庭食品安全】\n{}", parts.join("\n\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_foodsafetyrules_basic() {
        let rules = FoodSafetyRules::new();
        assert_eq!(rules.metadata().name, "家庭食品安全");
        assert!(!rules.shopping().is_empty());
        assert!(!rules.storage().is_empty());
        assert!(!rules.cooking().is_empty());
        assert!(!rules.hygiene().is_empty());
    }

    #[test]
    fn test_foodsafetyrules_validation() {
        let rules = FoodSafetyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("food_safety"));
    }

    #[test]
    fn test_foodsafetyrules_explain() {
        let rules = FoodSafetyRules::new();
        let e = rules.explain();
        assert!(e.contains("选购"));
        assert!(e.contains("储存"));
        assert!(e.contains("烹调"));
    }
}
