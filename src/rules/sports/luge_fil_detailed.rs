//! 雪橇FIL详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: LugeFilDetailedRules, name: "雪橇FIL详细", desc: "FIL雪橇详细规则", origin: "国际", tags: ["体育", "冬季"] }
impl LugeFilDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["单人双人团体"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["重量计时"]
    }
}
impl Rule for LugeFilDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("luge_fil_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "雪橇FIL详细",
            &[("项目", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = LugeFilDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
