//! 慰问书信措辞
//!
//! 写慰问信函的措辞、结构表达关心

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SympathyMessageGuideRules,
    name: "慰问书信措辞",
    desc: "写慰问信函的措辞、结构表达关心",
    origin: "中国",
    tags: ["社交", "慰问信", "措辞", "书面"]
}

impl SympathyMessageGuideRules {
    /// 书面慰问
    pub fn letter(&self) -> Vec<&'static str> {
        vec!["措辞真诚体贴", "开场表关心", "中段安慰", "结尾致祝福"]
    }

    /// 避免失言
    pub fn avoid(&self) -> Vec<&'static str> {
        vec!["不问事故细节", "不催促振作", "不说刺耳话", "低调为怀"]
    }

    /// 具体心意
    pub fn sincere(&self) -> Vec<&'static str> {
        vec!["可提有帮忙", "说愿陪伴", "真情感人", "不空说文腔"]
    }

    /// 落款时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec!["及时写信", "落款简洁", "卡片便笺皆可", "贵在真心"]
    }
}

impl Rule for SympathyMessageGuideRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("sympathy")
    }

    fn explain(&self) -> String {
        format!(
            "【慰问书信措辞】\n{}",
            [
                format!(
                    "书面慰问：\\n{}",
                    self.letter()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "避免失言：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "具体心意：\\n{}",
                    self.sincere()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "落款时机：\\n{}",
                    self.timing()
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
    fn test_sympathymessageguiderules_basic() {
        let rules = SympathyMessageGuideRules::new();
        assert_eq!(rules.metadata().name, "慰问书信措辞");
        assert!(!rules.letter().is_empty());
        assert!(!rules.avoid().is_empty());
        assert!(!rules.sincere().is_empty());
        assert!(!rules.timing().is_empty());
    }

    #[test]
    fn test_sympathymessageguiderules_validation() {
        let rules = SympathyMessageGuideRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("sympathy"));
    }

    #[test]
    fn test_sympathymessageguiderules_explain() {
        let rules = SympathyMessageGuideRules::new();
        let e = rules.explain();
        assert!(e.contains("书面慰问"));
        assert!(e.contains("避免失言"));
        assert!(e.contains("具体心意"));
    }
}
