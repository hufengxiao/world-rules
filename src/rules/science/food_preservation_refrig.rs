//! 食物保鲜冷藏
//!
//! 冰箱冷藏冷冻、食物保鲜与操作

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FoodPreservationRefrigRules,
    name: "食物保鲜冷藏",
    desc: "冰箱冷藏冷冻、食物保鲜与操作",
    origin: "生活",
    tags: ["科学", "保鲜", "冷藏", "食物"]
}

impl FoodPreservationRefrigRules {
    /// 冷藏分区
    pub fn zone(&self) -> Vec<&'static str> {
        vec!["易腐放冷藏", "肉蛋下层安全", "蔬果保鲜盒", "生熟分开"]
    }

    /// 冷冻保存
    pub fn freeze(&self) -> Vec<&'static str> {
        vec!["肉禽分装冷冻", "标注日期", "解冻用冷藏", "勿反复冻"]
    }

    /// 保味道
    pub fn fresh(&self) -> Vec<&'static str> {
        vec!["剩菜及时冷藏", "不用旧盒串味", "蔬果保鲜", "及时食用"]
    }

    /// 卫生操作
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec!["取放后盖紧", "定期清理冰箱", "变质食品丢弃", "生熟案板分开"]
    }
}

impl Rule for FoodPreservationRefrigRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("preservation")
    }

    fn explain(&self) -> String {
        format!(
            "【食物保鲜冷藏】\n{}",
            [
                format!(
                    "冷藏分区：\\n{}",
                    self.zone()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "冷冻保存：\\n{}",
                    self.freeze()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保味道：\\n{}",
                    self.fresh()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "卫生操作：\\n{}",
                    self.hygiene()
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
    fn test_foodpreservationrefrigrules_basic() {
        let rules = FoodPreservationRefrigRules::new();
        assert_eq!(rules.metadata().name, "食物保鲜冷藏");
        assert!(!rules.zone().is_empty());
        assert!(!rules.freeze().is_empty());
        assert!(!rules.fresh().is_empty());
        assert!(!rules.hygiene().is_empty());
    }

    #[test]
    fn test_foodpreservationrefrigrules_validation() {
        let rules = FoodPreservationRefrigRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("preservation"));
    }

    #[test]
    fn test_foodpreservationrefrigrules_explain() {
        let rules = FoodPreservationRefrigRules::new();
        let e = rules.explain();
        assert!(e.contains("冷藏分区"));
        assert!(e.contains("冷冻保存"));
        assert!(e.contains("保味道"));
    }
}
