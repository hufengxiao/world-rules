//! 保释法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BailLawRules, name: "保释法", desc: "保释法律规则", origin: "国际", tags: ["法律", "刑事"] }
impl BailLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["保释条件"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["保释权"]
    }
}
impl Rule for BailLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("bail_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "保释法",
            &[("条件", &self.section_0()), ("权利", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BailLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
