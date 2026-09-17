//! 赡养义务
//!
//! 成年子女赡养父母的经济、生活与精神义务

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ElderlySupportObligationRules,
    name: "赡养义务",
    desc: "成年子女赡养父母的经济、生活与精神义务",
    origin: "中国",
    tags: ["法律", "赡养", "父母", "家庭"]
}

impl ElderlySupportObligationRules {
    /// 法定义务
    pub fn duty(&self) -> Vec<&'static str> {
        vec![
            "成年子女赡养父母",
            "提供必要经济扶助",
            "生活照料",
            "给予精神慰藉",
        ]
    }

    /// 照护内容
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "保障基本生活所需",
            "疾病照护尽责任",
            "尊重父母意愿",
            "常回家看看陪伴",
        ]
    }

    /// 共同责任
    pub fn share(&self) -> Vec<&'static str> {
        vec![
            "多子女合理分担",
            "可与父母协商赡养方式",
            "经济困难依能力",
            "夫妻共同承担对双方父母",
        ]
    }

    /// 维权与履行
    pub fn perform(&self) -> Vec<&'static str> {
        vec![
            "遗弃拒绝赡养违法",
            "父母可依法主张",
            "倡导敬老爱老",
            "保障老人晚年",
        ]
    }
}

impl Rule for ElderlySupportObligationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("elderly_support")
    }

    fn explain(&self) -> String {
        format!(
            "【赡养义务】\n{}",
            [
                format!(
                    "法定义务：\\n{}",
                    self.duty()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "照护内容：\\n{}",
                    self.care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "共同责任：\\n{}",
                    self.share()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权与履行：\\n{}",
                    self.perform()
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
    fn test_elderlysupportobligationrules_basic() {
        let rules = ElderlySupportObligationRules::new();
        assert_eq!(rules.metadata().name, "赡养义务");
        assert!(!rules.duty().is_empty());
        assert!(!rules.care().is_empty());
        assert!(!rules.share().is_empty());
        assert!(!rules.perform().is_empty());
    }

    #[test]
    fn test_elderlysupportobligationrules_validation() {
        let rules = ElderlySupportObligationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("elderly_support"));
    }

    #[test]
    fn test_elderlysupportobligationrules_explain() {
        let rules = ElderlySupportObligationRules::new();
        let e = rules.explain();
        assert!(e.contains("法定义务"));
        assert!(e.contains("照护内容"));
        assert!(e.contains("共同责任"));
    }
}
