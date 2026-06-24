//! 中秋节礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MidAutumnRules, name: "中秋节礼仪", desc: "中秋节传统礼仪", origin: "中国", tags: ["社交", "节日"] }
impl MidAutumnRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["赏月习俗"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["送月饼礼节"]
    }
}
impl Rule for MidAutumnRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("mid_autumn")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中秋节礼仪",
            &[("赏月", &self.section_0()), ("月饼", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MidAutumnRules::new();
        assert!(!r.explain().is_empty());
    }
}
