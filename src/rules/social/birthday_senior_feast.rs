//! 寿宴礼仪
//!
//! 长辈寿宴的祝寿、礼物与席间礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BirthdaySeniorFeastRules,
    name: "寿宴礼仪",
    desc: "长辈寿宴的祝寿、礼物与席间礼仪",
    origin: "中国",
    tags: ["社交", "寿宴", "祝寿", "长辈"]
}

impl BirthdaySeniorFeastRules {
    /// 祝寿心意
    pub fn blessing(&self) -> Vec<&'static str> {
        vec!["真诚表达祝福", "祝健康长寿", "问候长辈身体", "祝福发自内心"]
    }

    /// 贺礼得当
    pub fn gift(&self) -> Vec<&'static str> {
        vec!["送实用心意礼", "选健康吉祥意", "礼轻情意重", "不攀比排场"]
    }

    /// 席间尊长
    pub fn seat(&self) -> Vec<&'static str> {
        vec!["长辈居上座", "敬酒敬寿星", "耐心听长谈", "不喧宾夺主"]
    }

    /// 珍惜团聚
    pub fn reunion(&self) -> Vec<&'static str> {
        vec!["陪伴多于礼物", "多听长辈往事", "合影留念", "孝在平常相处"]
    }
}

impl Rule for BirthdaySeniorFeastRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("birthday_feast")
    }

    fn explain(&self) -> String {
        format!(
            "【寿宴礼仪】\n{}",
            [
                format!(
                    "祝寿心意：\\n{}",
                    self.blessing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "贺礼得当：\\n{}",
                    self.gift()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "席间尊长：\\n{}",
                    self.seat()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "珍惜团聚：\\n{}",
                    self.reunion()
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
    fn test_birthdayseniorfeastrules_basic() {
        let rules = BirthdaySeniorFeastRules::new();
        assert_eq!(rules.metadata().name, "寿宴礼仪");
        assert!(!rules.blessing().is_empty());
        assert!(!rules.gift().is_empty());
        assert!(!rules.seat().is_empty());
        assert!(!rules.reunion().is_empty());
    }

    #[test]
    fn test_birthdayseniorfeastrules_validation() {
        let rules = BirthdaySeniorFeastRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("birthday_feast"));
    }

    #[test]
    fn test_birthdayseniorfeastrules_explain() {
        let rules = BirthdaySeniorFeastRules::new();
        let e = rules.explain();
        assert!(e.contains("祝寿心意"));
        assert!(e.contains("贺礼得当"));
        assert!(e.contains("席间尊长"));
    }
}
