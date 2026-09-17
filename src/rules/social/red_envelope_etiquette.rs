//! 红包礼仪
//!
//! 逢年过节派发收受红包的分寸与礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RedEnvelopeEtiquetteRules,
    name: "红包礼仪",
    desc: "逢年过节派发收受红包的分寸与礼节",
    origin: "中国",
    tags: ["社交", "礼仪", "红包", "节日"]
}

impl RedEnvelopeEtiquetteRules {
    /// 发派分寸
    pub fn give(&self) -> Vec<&'static str> {
        vec![
            "金额量力与关系相宜",
            "祝福寓于红包",
            "当着长辈恭敬送出",
            "数额以吉利为主",
        ]
    }

    /// 收受礼貌
    pub fn receive(&self) -> Vec<&'static str> {
        vec![
            "双手接拿表谢意",
            "当面感谢不推诿过度",
            "不当众拆数",
            "妥善装好",
        ]
    }

    /// 电子红包
    pub fn digital(&self) -> Vec<&'static str> {
        vec![
            "电子红包及时查收",
            "抢到后随手道谢",
            "不攀比数额",
            "群发注意礼貌",
        ]
    }

    /// 分寸得体
    pub fn measure(&self) -> Vec<&'static str> {
        vec![
            "不过分攀比不炫富",
            "视亲疏定亲厚",
            "敬老基本祝福",
            "不图红包失礼",
        ]
    }
}

impl Rule for RedEnvelopeEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("red_envelope")
    }

    fn explain(&self) -> String {
        format!(
            "【红包礼仪】\n{}",
            [
                format!(
                    "发派分寸：\\n{}",
                    self.give()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "收受礼貌：\\n{}",
                    self.receive()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "电子红包：\\n{}",
                    self.digital()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "分寸得体：\\n{}",
                    self.measure()
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
    fn test_redenvelopeetiquetterules_basic() {
        let rules = RedEnvelopeEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "红包礼仪");
        assert!(!rules.give().is_empty());
        assert!(!rules.receive().is_empty());
        assert!(!rules.digital().is_empty());
        assert!(!rules.measure().is_empty());
    }

    #[test]
    fn test_redenvelopeetiquetterules_validation() {
        let rules = RedEnvelopeEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("red_envelope"));
    }

    #[test]
    fn test_redenvelopeetiquetterules_explain() {
        let rules = RedEnvelopeEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("发派分寸"));
        assert!(e.contains("收受礼貌"));
        assert!(e.contains("电子红包"));
    }
}
