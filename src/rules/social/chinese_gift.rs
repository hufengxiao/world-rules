//! 中国送礼礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseGiftRules, name: "中国送礼礼仪", desc: "中国传统送礼礼仪", origin: "中国", tags: ["社交", "送礼"] }
impl ChineseGiftRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["钟梨伞"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["婚丧乔迁"]
    }
}
impl Rule for ChineseGiftRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_gift")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国送礼礼仪",
            &[("禁忌", &self.section_0()), ("场合", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseGiftRules::new();
        assert!(!r.explain().is_empty());
    }
}
