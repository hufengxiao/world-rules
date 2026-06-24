//! 纳米技术详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: NanotechnologyDetailedRules, name: "纳米技术详细定律", desc: "纳米技术定律", origin: "国际", tags: ["科学", "材料"] }
impl NanotechnologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["自组装CVD"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["量子尺寸"]
    }
}
impl Rule for NanotechnologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("nanotechnology_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "纳米技术详细定律",
            &[("制备", &self.section_0()), ("性质", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = NanotechnologyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
