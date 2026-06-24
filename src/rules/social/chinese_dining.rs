//! 中餐礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseDiningRules, name: "中餐礼仪", desc: "中国传统餐桌礼仪", origin: "中国", tags: ["社交", "餐桌"] }
impl ChineseDiningRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["主位客位"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["筷子禁忌"]
    }
}
impl Rule for ChineseDiningRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_dining")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中餐礼仪",
            &[("座次", &self.section_0()), ("筷子", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseDiningRules::new();
        assert!(!r.explain().is_empty());
    }
}
