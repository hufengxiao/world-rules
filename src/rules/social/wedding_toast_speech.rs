//! 婚礼敬酒致辞
//!
//! 婚礼敬酒顺序、致辞与祝福分寸

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WeddingToastSpeechRules,
    name: "婚礼敬酒致辞",
    desc: "婚礼敬酒顺序、致辞与祝福分寸",
    origin: "中国",
    tags: ["社交", "婚礼", "敬酒", "致辞"]
}

impl WeddingToastSpeechRules {
    /// 敬酒顺序
    pub fn order(&self) -> Vec<&'static str> {
        vec!["新人向长辈敬", "再敬宾客", "伴郎随后跟上", "依席进行"]
    }

    /// 致辞得体
    pub fn speech(&self) -> Vec<&'static str> {
        vec!["简短温情", "祝福新婚", "不冗长抢话", "语带真诚"]
    }

    /// 酒杯敬法
    pub fn glass(&self) -> Vec<&'static str> {
        vec!["举杯共饮", "碰杯轻响", "适度为饮", "不劝酒过量"]
    }

    /// 应对敬酒
    pub fn receive(&self) -> Vec<&'static str> {
        vec!["受敬起身", "回敬祝福", "不胜酒力请人代", "礼貌周全"]
    }
}

impl Rule for WeddingToastSpeechRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("wedding_toast")
    }

    fn explain(&self) -> String {
        format!(
            "【婚礼敬酒致辞】\n{}",
            [
                format!(
                    "敬酒顺序：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "致辞得体：\\n{}",
                    self.speech()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "酒杯敬法：\\n{}",
                    self.glass()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应对敬酒：\\n{}",
                    self.receive()
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
    fn test_weddingtoastspeechrules_basic() {
        let rules = WeddingToastSpeechRules::new();
        assert_eq!(rules.metadata().name, "婚礼敬酒致辞");
        assert!(!rules.order().is_empty());
        assert!(!rules.speech().is_empty());
        assert!(!rules.glass().is_empty());
        assert!(!rules.receive().is_empty());
    }

    #[test]
    fn test_weddingtoastspeechrules_validation() {
        let rules = WeddingToastSpeechRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("wedding_toast"));
    }

    #[test]
    fn test_weddingtoastspeechrules_explain() {
        let rules = WeddingToastSpeechRules::new();
        let e = rules.explain();
        assert!(e.contains("敬酒顺序"));
        assert!(e.contains("致辞得体"));
        assert!(e.contains("酒杯敬法"));
    }
}
