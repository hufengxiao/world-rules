//! 参加婚礼观礼
//!
//! 出席婚礼衣着、入场与观礼礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WeddingCeremonyGuestRules,
    name: "参加婚礼观礼",
    desc: "出席婚礼衣着、入场与观礼礼节",
    origin: "中国",
    tags: ["社交", "婚礼", "观礼", "宾客"]
}

impl WeddingCeremonyGuestRules {
    /// 着装得体
    pub fn attire(&self) -> Vec<&'static str> {
        vec!["穿着整洁正式", "不抢新娘风头", "色彩庄重搭配", "大方面对"]
    }

    /// 准时到场
    pub fn punctuality(&self) -> Vec<&'static str> {
        vec!["提前到场就座", "不迟到误仪式", "手机静音", "配合流程"]
    }

    /// 观礼分寸
    pub fn ceremony(&self) -> Vec<&'static str> {
        vec!["仪式期间安静", "不挡摄影机位", "鼓掌适时", "尊重新人"]
    }

    /// 道贺离场
    pub fn congratulate(&self) -> Vec<&'static str> {
        vec!["向新人道贺", "送祝福", "不喧宾夺主", "尽兴而归"]
    }
}

impl Rule for WeddingCeremonyGuestRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("wedding_guest")
    }

    fn explain(&self) -> String {
        format!(
            "【参加婚礼观礼】\n{}",
            [
                format!(
                    "着装得体：\\n{}",
                    self.attire()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "准时到场：\\n{}",
                    self.punctuality()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "观礼分寸：\\n{}",
                    self.ceremony()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "道贺离场：\\n{}",
                    self.congratulate()
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
    fn test_weddingceremonyguestrules_basic() {
        let rules = WeddingCeremonyGuestRules::new();
        assert_eq!(rules.metadata().name, "参加婚礼观礼");
        assert!(!rules.attire().is_empty());
        assert!(!rules.punctuality().is_empty());
        assert!(!rules.ceremony().is_empty());
        assert!(!rules.congratulate().is_empty());
    }

    #[test]
    fn test_weddingceremonyguestrules_validation() {
        let rules = WeddingCeremonyGuestRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("wedding_guest"));
    }

    #[test]
    fn test_weddingceremonyguestrules_explain() {
        let rules = WeddingCeremonyGuestRules::new();
        let e = rules.explain();
        assert!(e.contains("着装得体"));
        assert!(e.contains("准时到场"));
        assert!(e.contains("观礼分寸"));
    }
}
