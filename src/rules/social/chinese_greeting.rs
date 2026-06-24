//! 中国问候礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseGreetingRules, name: "中国问候礼仪", desc: "中国传统问候礼仪", origin: "中国", tags: ["社交", "问候"] }
impl ChineseGreetingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["拱手礼"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["敬称谦称"]
    }
}
impl Rule for ChineseGreetingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_greeting")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国问候礼仪",
            &[("拱手", &self.section_0()), ("称呼", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseGreetingRules::new();
        assert!(!r.explain().is_empty());
    }
}
