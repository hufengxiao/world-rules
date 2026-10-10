//! 乳糖不耐受护理
//!
//! 乳糖不耐受人群的饮食替代与营养补充要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LactoseIntoleranceCareRules,
    name: "乳糖不耐受护理",
    desc: "乳糖不耐受人群的饮食替代与营养补充要点",
    origin: "中国",
    tags: ["消化", "乳糖", "饮食"]
}

impl LactoseIntoleranceCareRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "饮奶后腹胀腹痛",
            "排气增多肠鸣腹泻",
            "多在半小时至两小时出现",
            "症状随摄入量加重",
        ]
    }

    /// 饮食替代
    pub fn substitute(&self) -> Vec<&'static str> {
        vec![
            "选无乳糖牛奶酸奶",
            "酸奶乳糖较低可试",
            "硬质奶酪乳糖少",
            "植物奶替代补钙源",
        ]
    }

    /// 摄入技巧
    pub fn intake(&self) -> Vec<&'static str> {
        vec![
            "少量多次逐渐适应",
            "随餐饮奶减轻反应",
            "避免空腹大量饮奶",
            "可选乳糖酶制剂",
        ]
    }

    /// 营养补充
    pub fn nutrition(&self) -> Vec<&'static str> {
        vec![
            "注意钙质足量摄入",
            "绿叶豆制品可补钙",
            "必要时补充维生素D",
            "生长发育期尤为重视",
        ]
    }
}

impl Rule for LactoseIntoleranceCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("lactose_intolerance_care")
    }

    fn explain(&self) -> String {
        format!(
            "【乳糖不耐受护理】\n{}",
            [
                format!(
                    "症状识别：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食替代：\\n{}",
                    self.substitute()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "摄入技巧：\\n{}",
                    self.intake()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "营养补充：\\n{}",
                    self.nutrition()
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
    fn test_lactoseintolerancecarerules_basic() {
        let rules = LactoseIntoleranceCareRules::new();
        assert_eq!(rules.metadata().name, "乳糖不耐受护理");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.substitute().is_empty());
        assert!(!rules.intake().is_empty());
        assert!(!rules.nutrition().is_empty());
    }

    #[test]
    fn test_lactoseintolerancecarerules_validation() {
        let rules = LactoseIntoleranceCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("lactose_intolerance_care")
        );
    }

    #[test]
    fn test_lactoseintolerancecarerules_explain() {
        let rules = LactoseIntoleranceCareRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("饮食替代"));
        assert!(e.contains("摄入技巧"));
    }
}
