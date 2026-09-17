//! 均衡膳食计划
//!
//! 安排均衡三餐、控量多样的健康膳食规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BalancedDietPlanRules,
    name: "均衡膳食计划",
    desc: "安排均衡三餐、控量多样的健康膳食规则",
    origin: "国际",
    tags: ["健康", "膳食", "均衡", "营养", "饮食"]
}

impl BalancedDietPlanRules {
    /// 饮食结构
    pub fn structure(&self) -> Vec<&'static str> {
        vec![
            "谷物蔬菜蛋白油脂均衡搭配",
            "每天进食多种类的食物",
            "以全谷物代替部分精制主食",
            "摄入足量蔬果与优质蛋白",
        ]
    }

    /// 餐次安排
    pub fn meals(&self) -> Vec<&'static str> {
        vec![
            "规律三餐不过度饥饿",
            "两餐之间适量加餐",
            "早餐不省略营养充足",
            "晚上减少过饱进食",
        ]
    }

    /// 控量原则
    pub fn portion(&self) -> Vec<&'static str> {
        vec![
            "使用适量份量进餐",
            "限制油脂糖与盐",
            "减少含糖饮料",
            "注重咀嚼细嚼慢咽",
        ]
    }

    /// 顺应个体
    pub fn individual(&self) -> Vec<&'static str> {
        vec![
            "依据年龄活动量调整",
            "特殊人群遵医嘱安排",
            "结合文化饮食习惯",
            "可持续坚持避免短期极端",
        ]
    }
}

impl Rule for BalancedDietPlanRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("balanced_diet")
    }

    fn explain(&self) -> String {
        format!(
            "【均衡膳食计划】\n{}",
            [
                format!(
                    "饮食结构：\\n{}",
                    self.structure()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "餐次安排：\\n{}",
                    self.meals()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "控量原则：\\n{}",
                    self.portion()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "顺应个体：\\n{}",
                    self.individual()
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
    fn test_balanceddietplanrules_basic() {
        let rules = BalancedDietPlanRules::new();
        assert_eq!(rules.metadata().name, "均衡膳食计划");
        assert!(!rules.structure().is_empty());
        assert!(!rules.meals().is_empty());
        assert!(!rules.portion().is_empty());
        assert!(!rules.individual().is_empty());
    }

    #[test]
    fn test_balanceddietplanrules_validation() {
        let rules = BalancedDietPlanRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("balanced_diet"));
    }

    #[test]
    fn test_balanceddietplanrules_explain() {
        let rules = BalancedDietPlanRules::new();
        let e = rules.explain();
        assert!(e.contains("饮食结构"));
        assert!(e.contains("餐次安排"));
        assert!(e.contains("控量原则"));
    }
}
