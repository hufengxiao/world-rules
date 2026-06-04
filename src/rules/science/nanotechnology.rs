//! 纳米技术定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: NanotechnologyRules, name: "纳米技术定律", desc: "纳米技术定律", origin: "国际", tags: ["科学", "材料"] }
impl NanotechnologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["自组装", "化学气相沉积"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["量子尺寸效应", "表面效应"]
    }
}
impl Rule for NanotechnologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("nanotechnology")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "纳米技术定律",
            &[("制备", &self.section_0()), ("性质", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = NanotechnologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
