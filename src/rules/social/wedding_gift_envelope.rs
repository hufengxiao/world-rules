//! 婚礼随礼红包
//!
//! 参加婚礼送红包的金额、形式与礼数

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WeddingGiftEnvelopeRules,
    name: "婚礼随礼红包",
    desc: "参加婚礼送红包的金额、形式与礼数",
    origin: "中国",
    tags: ["社交", "红包", "随礼", "婚礼"]
}

impl WeddingGiftEnvelopeRules {
    /// 金额心意
    pub fn amount(&self) -> Vec<&'static str> {
        vec!["随礼量力而行", "双数为吉", "依亲疏而定", "心意为主"]
    }

    /// 红包形式
    pub fn form(&self) -> Vec<&'static str> {
        vec!["用红包装", "写上姓名", "递交给收礼人", "礼貌交送"]
    }

    /// 当面赠礼
    pub fn present(&self) -> Vec<&'static str> {
        vec!["到签到台登记", "祝福一并口出", "不直接塞衣袋", "得体大方"]
    }

    /// 礼尚往来
    pub fn reciprocity(&self) -> Vec<&'static str> {
        vec!["记住人情往来", "还礼相当", "礼轻情重", "情谊为重"]
    }
}

impl Rule for WeddingGiftEnvelopeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("wedding_envelope")
    }

    fn explain(&self) -> String {
        format!(
            "【婚礼随礼红包】\n{}",
            [
                format!(
                    "金额心意：\\n{}",
                    self.amount()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "红包形式：\\n{}",
                    self.form()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "当面赠礼：\\n{}",
                    self.present()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "礼尚往来：\\n{}",
                    self.reciprocity()
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
    fn test_weddinggiftenveloperules_basic() {
        let rules = WeddingGiftEnvelopeRules::new();
        assert_eq!(rules.metadata().name, "婚礼随礼红包");
        assert!(!rules.amount().is_empty());
        assert!(!rules.form().is_empty());
        assert!(!rules.present().is_empty());
        assert!(!rules.reciprocity().is_empty());
    }

    #[test]
    fn test_weddinggiftenveloperules_validation() {
        let rules = WeddingGiftEnvelopeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("wedding_envelope"));
    }

    #[test]
    fn test_weddinggiftenveloperules_explain() {
        let rules = WeddingGiftEnvelopeRules::new();
        let e = rules.explain();
        assert!(e.contains("金额心意"));
        assert!(e.contains("红包形式"));
        assert!(e.contains("当面赠礼"));
    }
}
