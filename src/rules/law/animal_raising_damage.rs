//! 饲养动物致害责任
//!
//! 饲养动物伤人毁物时的赔偿责任与维权要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AnimalRaisingDamageRules,
    name: "饲养动物致害责任",
    desc: "饲养动物伤人毁物时的赔偿责任与维权要点",
    origin: "中国",
    tags: ["法律", "宠物", "侵权", "责任"]
}

impl AnimalRaisingDamageRules {
    /// 责任主体
    pub fn subject(&self) -> Vec<&'static str> {
        vec![
            "饲养人管理人担责",
            "动物伤人造损失",
            "拴养尽看护义务",
            "逃逸后仍负责",
        ]
    }

    /// 免责情形
    pub fn defense(&self) -> Vec<&'static str> {
        vec![
            "被侵权人故意引发",
            "重大过失可减责",
            "违规逗惹折减",
            "第三方过错另论",
        ]
    }

    /// 赔偿内容
    pub fn damages(&self) -> Vec<&'static str> {
        vec!["医疗费用", "误工损失", "财产受损赔偿", "精神损害酌定"]
    }

    /// 举证维权
    pub fn evidence(&self) -> Vec<&'static str> {
        vec![
            "保留现场照片",
            "医疗票据",
            "报警或调解记录",
            "及时协商或诉讼",
        ]
    }
}

impl Rule for AnimalRaisingDamageRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("animal_raising")
    }

    fn explain(&self) -> String {
        format!(
            "【饲养动物致害责任】\n{}",
            [
                format!(
                    "责任主体：\\n{}",
                    self.subject()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "免责情形：\\n{}",
                    self.defense()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "赔偿内容：\\n{}",
                    self.damages()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "举证维权：\\n{}",
                    self.evidence()
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
    fn test_animalraisingdamagerules_basic() {
        let rules = AnimalRaisingDamageRules::new();
        assert_eq!(rules.metadata().name, "饲养动物致害责任");
        assert!(!rules.subject().is_empty());
        assert!(!rules.defense().is_empty());
        assert!(!rules.damages().is_empty());
        assert!(!rules.evidence().is_empty());
    }

    #[test]
    fn test_animalraisingdamagerules_validation() {
        let rules = AnimalRaisingDamageRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("animal_raising"));
    }

    #[test]
    fn test_animalraisingdamagerules_explain() {
        let rules = AnimalRaisingDamageRules::new();
        let e = rules.explain();
        assert!(e.contains("责任主体"));
        assert!(e.contains("免责情形"));
        assert!(e.contains("赔偿内容"));
    }
}
