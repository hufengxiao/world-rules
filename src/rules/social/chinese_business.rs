//! 中国商务礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseBusinessRules, name: "中国商务礼仪", desc: "中国传统商务礼仪", origin: "中国", tags: ["社交", "商务"] }
impl ChineseBusinessRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["递名片"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["敬酒礼节"]
    }
}
impl Rule for ChineseBusinessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_business")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国商务礼仪",
            &[("名片", &self.section_0()), ("酒桌", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseBusinessRules::new();
        assert!(!r.explain().is_empty());
    }
}
