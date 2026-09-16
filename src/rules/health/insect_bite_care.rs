//! 常见蚊虫叮咬处理
//!
//! 蚊虫、蜂等叮咬后的止痒消毒与安全隐患识别

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: InsectBiteCareRules,
    name: "常见蚊虫叮咬处理",
    desc: "蚊虫、蜂等叮咬后的止痒消毒与安全隐患识别",
    origin: "医学",
    tags: ["健康", "蚊虫", "叮咬", "止痒", "护理"]
}

impl InsectBiteCareRules {
    /// 基本处理
    pub fn basic(&self) -> Vec<&'static str> {
        vec![
            "避免抓挠防止破损感染",
            "用冷敷缓解红肿痒",
            "涂抹无刺激止痒药膏",
            "保持叮咬处清洁干燥",
        ]
    }

    /// 种类应对
    pub fn types(&self) -> Vec<&'static str> {
        vec![
            "蚊虫叮咬一般外用即可缓解",
            "有刺伤的蜂类先小心移除",
            "蜱叮咬勿硬拔需就医处理",
            "不挤压叮咬部位以防毒液扩散",
        ]
    }

    /// 过敏警示
    pub fn allergy(&self) -> Vec<&'static str> {
        vec![
            "大面积红肿蔓延就医",
            "出现呼吸困难或喉头紧立即急诊",
            "伴发热、头晕、全身症状就医",
            "对蜂毒有过敏史者警惕",
        ]
    }

    /// 预防
    pub fn prevention(&self) -> Vec<&'static str> {
        vec![
            "野外穿长袖长裤装防蚊衣",
            "使用驱蚊剂与蚊帐",
            "清理积水减少蚊虫滋生",
            "避免涂抹浓香吸引蜂类",
        ]
    }
}

impl Rule for InsectBiteCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("insect_bite")
    }

    fn explain(&self) -> String {
        format!(
            "【常见蚊虫叮咬处理】\n{}",
            [
                format!(
                    "基本处理：\\n{}",
                    self.basic()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "种类应对：\\n{}",
                    self.types()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "过敏警示：\\n{}",
                    self.allergy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "预防：\\n{}",
                    self.prevention()
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
    fn test_insectbitecarerules_basic() {
        let rules = InsectBiteCareRules::new();
        assert_eq!(rules.metadata().name, "常见蚊虫叮咬处理");
        assert!(!rules.basic().is_empty());
        assert!(!rules.types().is_empty());
        assert!(!rules.allergy().is_empty());
        assert!(!rules.prevention().is_empty());
    }

    #[test]
    fn test_insectbitecarerules_validation() {
        let rules = InsectBiteCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("insect_bite"));
    }

    #[test]
    fn test_insectbitecarerules_explain() {
        let rules = InsectBiteCareRules::new();
        let e = rules.explain();
        assert!(e.contains("基本处理"));
        assert!(e.contains("种类应对"));
        assert!(e.contains("过敏警示"));
    }
}
